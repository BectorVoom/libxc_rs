//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1000/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1000(t35379: f64, t4643: f64, t7486: f64, t2095: f64, t1427: f64, t31491: f64, t7381: f64, t1345: f64, t1983: f64, t7380: f64, t1462: f64, t7614: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35380 = 0.31448092289604152068e-3_f64 * t35379;
    let t35383 = t4643 * t7486;
    let t35384 = t2095 * t35383;
    let t35385 = 0.305625e-1_f64 * t35384;
    let t35387 = t31491 * t7381 * t1427;
    let t35388 = t35387 / 8.0_f64;
    let t35390 = t7380 * t1983 * t1345;
    let t35391 = t35390 / 32.0_f64;
    let t35392 = t7614 * t1462;
    (t35380, t35383, t35385, t35388, t35391, t35392)
}
