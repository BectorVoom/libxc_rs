//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 692/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk692(t5: f64, t114: f64, t1923: f64, t2048: f64, t7343: f64, t7351: f64, t7702: f64, t7706: f64, t7709: f64, t7964: f64, t117: f64, t1843: f64, t2055: f64, t7370: f64, t7738: f64) -> (f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t115 = 1.0_f64 < t114;
    let t7968 = piecewise3(t8, 0.0_f64, t7702 * t2048 / 3.0_f64 - 5.0_f64 / 3.0_f64 * t7343 * t7706 - 2.0_f64 / 3.0_f64 * t7709 * t2048 - t7351 + t1923 * t7964 / 3.0_f64);
    let t7969 = t7968 * t117;
    let t7978 = t1843 * t2055;
    let t7983 = piecewise3(t115, 0.0_f64, -t7370 - t7738 / 4.0_f64);
    (t7968, t7969, t7978, t7983)
}
