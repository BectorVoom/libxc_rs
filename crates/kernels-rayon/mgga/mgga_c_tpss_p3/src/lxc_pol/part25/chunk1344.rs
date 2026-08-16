//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1344/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1344(t5: f64, t71386: f64, t71411: f64, t71431: f64, t71460: f64, t71487: f64, t71499: f64, t71520: f64, t71544: f64, t117: f64, t1338: f64, t13546: f64, t13565: f64, t1799: f64, t18898: f64, t20289: f64, t20294: f64, t25232: f64, t3537: f64, t42710: f64, t4674: f64, t50656: f64, t5801: f64, t5815: f64, t645: f64, t67541: f64, t69023: f64, t71308: f64, t71344: f64, t71374: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t71548 = piecewise3(t8, 0.0_f64, t71386 + t71411 + t71431 + t71460 + t71487 + t71499 + t71520 + t71544);
    let t71549 = t71548 * t117;
    let t71574 = 4.0_f64 * t1338 * t67541 + 4.0_f64 * t1338 * t71344 + 2.0_f64 * t13546 * t5801 + 2.0_f64 * t13565 * t5815 + 2.0_f64 * t1799 * t42710 + 2.0_f64 * t1799 * t50656 + 4.0_f64 * t1799 * t69023 + 2.0_f64 * t18898 * t4674 + 4.0_f64 * t20289 * t3537 + 2.0_f64 * t20294 * t4674 + 4.0_f64 * t25232 * t3537 + 2.0_f64 * t645 * t71308 + 2.0_f64 * t71374 + t71549;
    (t71549, t71574)
}
