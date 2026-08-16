//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1352/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1352(t71776: f64, t71787: f64, t71798: f64, t71807: f64, t1232: f64, t1265: f64, t13884: f64, t13889: f64, t13941: f64, t1639: f64, t1772: f64, t1773: f64, t18483: f64, t18490: f64, t18511: f64, t18967: f64, t19509: f64, t19535: f64, t19540: f64, t19542: f64, t20154: f64, t20157: f64, t20183: f64, t20190: f64, t20206: f64, t21804: f64, t21827: f64, t21830: f64, t3260: f64, t4459: f64, t4516: f64, t4517: f64, t520: f64, t522: f64, t5448: f64, t5739: f64, t5740: f64, t5745: f64, t5918: f64, t5921: f64, t60653: f64, t6419: f64, t67006: f64, t67061: f64, t69663: f64, t69667: f64, t69738: f64, t71725: f64, t71748: f64) -> (f64, f64) {
    let t71809 = t71776 + t71787 + t71798 + t71807;
    let t71823 = t19540 * t18967 * t69738 + 6.0_f64 * t19540 * t67006 * t69663 - 6.0_f64 * t19540 * t20190 * t69667 - 4.0_f64 * t19540 * t71725 * t19542 + 4.0_f64 * t18483 * t21827 + 4.0_f64 * t19509 * t20206 + t5739 * t5745 * t21804 * t1232 * t520 - 2.0_f64 * t20157 * t4517 + 4.0_f64 * t5739 * t5740 * t6419 * t4516 + 12.0_f64 * t60653 * t67061 * t19535 + 4.0_f64 * t5921 * t13884 - 2.0_f64 * t5739 * t18511 * t71748 * t3260 - t5921 * t13941 + 4.0_f64 * t19509 * t20183 + 2.0_f64 * t5739 * t5745 * t20154 * t1639 * t520 + 2.0_f64 * t5739 * t5745 * t6419 * t4459 * t520 - t1772 * t1773 * t522 * t71809 + 2.0_f64 * t5739 * t5740 * t5918 * t5448 - 6.0_f64 * t5739 * t18490 * t21830 * t1265 + 2.0_f64 * t5921 * t13889;
    (t71809, t71823)
}
