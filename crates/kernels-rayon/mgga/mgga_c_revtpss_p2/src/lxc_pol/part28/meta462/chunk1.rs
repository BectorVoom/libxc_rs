//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1762/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1762(t2035: f64, t25188: f64, t531: f64, t7311: f64, t7238: f64, t2014: f64, t7312: f64, t7315: f64, t1310: f64, t1453: f64, t1932: f64, t2007: f64, t2320: f64, t2328: f64, t25078: f64, t25085: f64, t25092: f64, t25095: f64, t25096: f64, t25169: f64, t25180: f64, t25182: f64, t25184: f64, t25186: f64, t3813: f64, t508: f64, t649: f64, t651: f64, t6983: f64, t7221: f64, t7231: f64) -> (f64, f64, f64, f64) {
    let t25189 = t25188 * t2035;
    let t25190 = t531 * t7311;
    let t25191 = t25190 * t7238;
    let t25193 = 6.0_f64 * t2014 * t25191;
    let t25194 = t7312 * t7315;
    let t25196 = 2.0_f64 * t2014 * t25194;
    let t25197 = -2.0_f64 * t1310 * t6983 + 2.0_f64 * t1453 * t7231 - t1932 * t3813 - t2007 * t2320 - 2.0_f64 * t2007 * t2328 - 2.0_f64 * t25078 * t651 - 2.0_f64 * t25096 * t508 - t25169 * t508 - 2.0_f64 * t649 * t7221 - t25085 + t25092 - t25095 + t25180 - t25182 - t25184 - t25186 + t25189 + t25193 - t25196;
    (t25190, t25191, t25194, t25197)
}
