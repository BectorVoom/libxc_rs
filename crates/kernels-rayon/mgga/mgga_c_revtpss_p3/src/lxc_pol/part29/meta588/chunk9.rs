//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1950/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1950(t28093: f64, t7349: f64, t26169: f64, t7702: f64, t28640: f64, t6954: f64, t1923: f64, t28089: f64, t7348: f64, t26205: f64, t101360: f64, t2048: f64, t25150: f64, t26172: f64, t7352: f64, t7964: f64, t95297: f64, t95314: f64, t95320: f64) -> f64 {
    let t101899 = 16.0_f64 / 9.0_f64 * t28093 * t7349;
    let t101901 = 16.0_f64 / 9.0_f64 * t7702 * t26169;
    let t101903 = 16.0_f64 / 9.0_f64 * t6954 * t28640;
    let t101906 = 16.0_f64 / 9.0_f64 * t1923 * t7348 * t28089;
    let t101907 = t7702 * t26205;
    let t101919 = -160.0_f64 / 9.0_f64 * t95297 - t101899 - t101901 - t101903 - t101906 + 88.0_f64 / 27.0_f64 * t101907 - 352.0_f64 / 27.0_f64 * t95314 - 80.0_f64 / 3.0_f64 * t95320 + t101360 * t2048 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t28093 * t7352 + t7702 * t26172 / 3.0_f64 + t25150 * t7964 / 3.0_f64;
    t101919
}
