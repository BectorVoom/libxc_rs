//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3424/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3424(t52163: f64, t52482: f64, t934: f64, t15390: f64, t52514: f64, t19056: f64, t2919: f64, t2923: f64, t6104: f64, t2927: f64, t1610: f64, t52214: f64) -> (f64, f64, f64, f64, f64) {
    let t64327 = 0.2069040516770936012e4_f64 * t52482 * t52163 * t934;
    let t64329 = 0.38596750796862084161e3_f64 * t52514 * t15390;
    let t64335 = 1.0_f64 * t19056 * t2919;
    let t64336 = t6104 * t2923;
    let t64338 = 0.16081979498692535067e2_f64 * t64336 * t2927;
    let t64340 = 2.0_f64 * t52214 * t1610;
    (t64327, t64329, t64335, t64338, t64340)
}
