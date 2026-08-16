//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 982/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk982(t24253: f64, t300: f64, t1733: f64, t20629: f64, t5063: f64, t6471: f64, t16840: f64, t6474: f64, t24220: f64, t3435: f64, t12248: f64, t5071: f64, t6449: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24255 = 0.19751673498613801407e-1_f64 * t300 * t24253;
    let t24257 = 3.0_f64 * t20629 * t1733;
    let t24259 = 3.0_f64 * t5063 * t6471;
    let t24261 = 0.48245938496077605201e2_f64 * t16840 * t6474;
    let t24262 = t24220 * t3435;
    let t24264 = 0.96491876992155210402e2_f64 * t12248 * t24262;
    let t24265 = t5071 * t6449;
    (t24255, t24257, t24259, t24261, t24264, t24265)
}
