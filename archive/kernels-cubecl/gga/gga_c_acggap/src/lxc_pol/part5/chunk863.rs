//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 863/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk863<F: Float>(t12295: F, t1233: F, t1004: F, t3832: F, t1244: F, t3243: F, t3088: F, t3089: F, t930: F, t3077: F, t3080: F, t1529: F, t851: F) -> (F, F, F, F, F, F) {
    let t12297 = F::cast_from(0.79025390195226139183e1_f64) * t12295 * t1233;
    let t12298 = t1004 * t3832;
    let t12301 = F::cast_from(0.39512695097613069592e1_f64) * t3243 * t1244;
    let t12305 = t3088 * t3089 * t930;
    let t12307 = t3077 * t3080;
    let t12309 = t851 * t1529;
    (t12297, t12298, t12301, t12305, t12307, t12309)
}
