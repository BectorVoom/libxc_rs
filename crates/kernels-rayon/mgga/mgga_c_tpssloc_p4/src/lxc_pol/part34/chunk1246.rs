//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1246/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1246(t102386: f64, t106731: f64, t106935: f64, t106956: f64, t107634: f64, t108004: f64, t108767: f64, t1268: f64, t1458: f64, t19451: f64, t20347: f64, t2039: f64, t27188: f64, t28002: f64, t28007: f64, t28951: f64, t33234: f64, t4028: f64, t5493: f64, t67001: f64, t7042: f64, t7676: f64, t7801: f64) -> f64 {
    let t108844 = 6.0_f64 * t102386 * t1458 + 6.0_f64 * t106731 * t2039 + 2.0_f64 * t106935 * t2039 + 6.0_f64 * t106956 * t2039 + 2.0_f64 * t107634 * t1268 + 6.0_f64 * t19451 * t7801 + 2.0_f64 * t20347 * t7042 + 2.0_f64 * t2039 * t67001 + 6.0_f64 * t27188 * t5493 + 12.0_f64 * t28002 * t7801 + 6.0_f64 * t28007 * t7801 + 6.0_f64 * t28951 * t4028 + 6.0_f64 * t28951 * t7676 + 6.0_f64 * t33234 * t5493 + 6.0_f64 * t108004 + t108767;
    t108844
}
