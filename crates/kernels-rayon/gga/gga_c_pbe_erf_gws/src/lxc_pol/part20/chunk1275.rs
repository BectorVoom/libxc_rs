//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1275/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1275(t1113: f64, t2118: f64, t3028: f64, t3972: f64, t3975: f64, t14767: f64, t3052: f64, t14657: f64, t8602: f64, t15149: f64, t3038: f64, t8716: f64) -> (f64, f64, f64, f64, f64) {
    let t56166 = t3972 * t3975 * t1113 * t2118 * t3028;
    let t56168 = t14767 * t3052;
    let t56170 = t14657 * t8602;
    let t56174 = t3972 * t3975 * t3038 * t15149;
    let t56176 = t14657 * t8716;
    (t56166, t56168, t56170, t56174, t56176)
}
