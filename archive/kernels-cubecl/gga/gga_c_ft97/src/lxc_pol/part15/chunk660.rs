//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 660/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk660<F: Float>(t5134: F, t681: F, t89: F, t332: F, t992: F, t2253: F, t5470: F, t5459: F, t10304: F, t4939: F, t5442: F, t5446: F, t8675: F) -> (F, F, F, F, F, F, F) {
    let t18746 = t89 * t681 * t5134;
    let t18798 = t332 * t992;
    let t18823 = t2253 * t5470;
    let t18825 = t2253 * t5459;
    let t18826 = t10304 * t4939;
    let t18874 = t2253 * t5442;
    let t18877 = t8675 * t5446;
    (t18746, t18798, t18823, t18825, t18826, t18874, t18877)
}
