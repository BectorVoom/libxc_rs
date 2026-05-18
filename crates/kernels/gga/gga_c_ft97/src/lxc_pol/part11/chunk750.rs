//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 750/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk750<F: Float>(t1882: F, t2587: F, t2409: F, t724: F, t773: F, t2614: F, t2581: F, t2469: F, t2526: F, t242: F, t2542: F, t761: F) -> (F, F, F, F, F, F, F) {
    let t10140 = t1882 * t2587;
    let t10143 = t724 * t773 * t2409;
    let t10146 = t1882 * t2614;
    let t10148 = t1882 * t2581;
    let t10150 = t2469 * t2526;
    let t10151 = t242 * t10150;
    let t10153 = t2542 * t761;
    (t10140, t10143, t10146, t10148, t10150, t10151, t10153)
}
