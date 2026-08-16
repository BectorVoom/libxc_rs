//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 948/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk948(t13969: f64, t4599: f64, t3039: f64, t3069: f64, t4669: f64, t10231: f64, t4338: f64, t973: f64, t4595: f64, t3130: f64, t3048: f64, t4571: f64) -> (f64, f64, f64, f64, f64) {
    let t13970 = t13969 * t4599;
    let t13972 = t3039 * t13970 / 2304.0_f64;
    let t13995 = t4669 * t3069;
    let t13998 = t10231 * t4338;
    let t14000 = t973 * t13998 / 324.0_f64;
    let t14025 = t13969 * t4595;
    let t14027 = t3130 * t14025 / 1152.0_f64;
    let t14049 = t3048 * t4571 / 648.0_f64;
    (t13972, t13995, t14000, t14027, t14049)
}
