//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1019/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1019<F: Float>(t2542: F, t2567: F, t2569: F, t10153: F, t2526: F, t10121: F, t2469: F, t10069: F, t9591: F, t2354: F, t446: F, t9582: F) -> (F, F, F, F, F, F) {
    let t41416 = t2542 * t2567;
    let t41417 = t41416 * t2569;
    let t41419 = t10153 * t2526;
    let t41421 = t2469 * t10121;
    let t41431 = t9591 * t10069;
    let t41433 = t446 * t2354 * t41431;
    let t41435 = t9582 * t10069;
    (t41417, t41419, t41421, t41431, t41433, t41435)
}
