//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 936/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk936(t11239: f64, t342: f64, t3145: f64, t334: f64, t368: f64, t365: f64, t3144: f64, t1043: f64, t3151: f64, t373: f64, t3153: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11240 = t342 * t11239;
    let t11243 = 1.0_f64 / t3145 / t368 / t334;
    let t11244 = t365 * t11243;
    let t11245 = t3144 * t11244;
    let t11246 = t11240 * t11245;
    let t11247 = t3151 * t1043;
    let t11248 = t373 * t11247;
    let t11249 = t3153 * t73;
    (t11240, t11243, t11244, t11246, t11247, t11248, t11249)
}
