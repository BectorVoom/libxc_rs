//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 961/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk961(t2047: f64, t28150: f64, t28089: f64, t7349: f64, t7702: f64, t7348: f64, t7719: f64, t1923: f64, t2048: f64, t25162: f64, t26170: f64, t26175: f64, t26182: f64, t26190: f64, t26207: f64, t28093: f64, t28133: f64, t28147: f64, t28154: f64, t6954: f64, t6963: f64, t7343: f64, t7352: f64, t7964: f64) -> f64 {
    let t28628 = t2047 * t28150;
    let t28635 = t2047 * t28089;
    let t28638 = t7702 * t7349;
    let t28640 = t7348 * t7719;
    let t28641 = t1923 * t28640;
    let t28649 = -5.0_f64 / 3.0_f64 * t7343 * t28133 - 8.0_f64 / 9.0_f64 * t26170 - 8.0_f64 / 9.0_f64 * t26190 + t26207 + 10.0_f64 * t26175 * t28147 + 10.0_f64 / 3.0_f64 * t25162 * t28628 + 10.0_f64 / 3.0_f64 * t28154 * t26182 + t6954 * t7964 / 3.0_f64 + t1923 * t28635 / 3.0_f64 - 8.0_f64 / 9.0_f64 * t28638 - 8.0_f64 / 9.0_f64 * t28641 - 2.0_f64 / 3.0_f64 * t6963 * t7964 + t28093 * t2048 / 3.0_f64 + t7702 * t7352 / 3.0_f64;
    t28649
}
