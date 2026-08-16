//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 690/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk690(t1170: f64, t1184: f64, t1186: f64, t19: f64, t27: f64, t498: f64, t123: f64, t497: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3301 = t1170 * t1184;
    let t3302 = 8.0_f64 * t3301;
    let t3304 = 8.0_f64 * t1170 * t1186;
    let t3305 = t19 * t27;
    let t3307 = 20.0_f64 * t3305 * t498;
    let t3308 = t497 * t123;
    (t3301, t3302, t3304, t3305, t3307, t3308)
}
