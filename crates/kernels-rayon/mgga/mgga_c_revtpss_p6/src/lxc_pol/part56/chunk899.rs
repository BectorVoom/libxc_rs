//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 899/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk899(t29343: f64, t29425: f64, t29451: f64, t29466: f64, t3: f64, t1461: f64, t1918: f64, t2170: f64, t28257: f64, t28259: f64, t28261: f64, t28263: f64, t28267: f64, t28270: f64, t28273: f64, t28275: f64, t28279: f64, t28282: f64, t573: f64, t5802: f64, t5805: f64, t7696: f64, t8245: f64, param_d: f64) -> (f64, f64, f64) {
    let t29468 = t29343 + t29425 + t29451 + t29466;
    let t29469 = t3 * t29468;
    let t29480 = param_d * t29468;
    let t29490 = 3.0_f64 * t1461 * t8245 + 3.0_f64 * t1918 * t7696 + 6.0_f64 * t2170 * t5802 + 3.0_f64 * t2170 * t5805 + t29480 * t573 + t28257 + t28259 + t28261 + t28263 + t28267 + t28270 + t28273 + t28275 + t28279 + t28282;
    (t29469, t29480, t29490)
}
