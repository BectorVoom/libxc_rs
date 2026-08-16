//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3225/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3225(t10428: f64, t5999: f64, t18544: f64, t2398: f64, t14440: f64, t4311: f64, t4537: f64, t775: f64, t14386: f64, t4308: f64, t39860: f64, t18498: f64, t2403: f64, t2404: f64, t39799: f64, t39807: f64, t39813: f64, t39818: f64, t39823: f64, t4541: f64, t4556: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t61177 = 4.0_f64 * t10428 * t5999;
    let t61178 = t2398 * t18544;
    let t61179 = 8.0_f64 * t61178;
    let t61180 = t4311 * t14440;
    let t61181 = 8.0_f64 * t61180;
    let t61182 = t775 * t4537;
    let t61190 = 16.0_f64 * t14386 * t4308;
    let t61191 = 0.11393789434848516922e-2_f64 * t39860;
    let t61192 = 24.0_f64 * t18498 * t2404 * t4541 - 12.0_f64 * t2403 * t4556 * t61182 + t39799 + t39807 - t39813 - t39818 - t39823 + t61177 + t61179 + t61181 + t61190 - t61191;
    (t61177, t61179, t61181, t61190, t61191, t61192)
}
