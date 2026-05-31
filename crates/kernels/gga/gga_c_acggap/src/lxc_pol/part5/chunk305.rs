//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 305/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk305<F: Float>(t1049: F, t347: F, t136: F, t357: F, t576: F, t137: F, t154: F) -> (F, F, F, F) {
    let t1050 = t1049 * t347;
    let t1053 = t576 * t136 * t357;
    let t1054 = t1053 / F::cast_from(6.0_f64);
    let t1055 = t154 * t137;
    (t1050, t1053, t1054, t1055)
}
