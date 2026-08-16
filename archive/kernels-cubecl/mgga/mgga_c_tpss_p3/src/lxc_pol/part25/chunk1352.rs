//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1352/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1352<F: Float>(t71776: F, t71787: F, t71798: F, t71807: F, t1232: F, t1265: F, t13884: F, t13889: F, t13941: F, t1639: F, t1772: F, t1773: F, t18483: F, t18490: F, t18511: F, t18967: F, t19509: F, t19535: F, t19540: F, t19542: F, t20154: F, t20157: F, t20183: F, t20190: F, t20206: F, t21804: F, t21827: F, t21830: F, t3260: F, t4459: F, t4516: F, t4517: F, t520: F, t522: F, t5448: F, t5739: F, t5740: F, t5745: F, t5918: F, t5921: F, t60653: F, t6419: F, t67006: F, t67061: F, t69663: F, t69667: F, t69738: F, t71725: F, t71748: F) -> (F, F) {
    let t71809 = t71776 + t71787 + t71798 + t71807;
    let t71823 = t19540 * t18967 * t69738 + F::cast_from(6.0_f64) * t19540 * t67006 * t69663 - F::cast_from(6.0_f64) * t19540 * t20190 * t69667 - F::cast_from(4.0_f64) * t19540 * t71725 * t19542 + F::cast_from(4.0_f64) * t18483 * t21827 + F::cast_from(4.0_f64) * t19509 * t20206 + t5739 * t5745 * t21804 * t1232 * t520 - F::cast_from(2.0_f64) * t20157 * t4517 + F::cast_from(4.0_f64) * t5739 * t5740 * t6419 * t4516 + F::cast_from(12.0_f64) * t60653 * t67061 * t19535 + F::cast_from(4.0_f64) * t5921 * t13884 - F::cast_from(2.0_f64) * t5739 * t18511 * t71748 * t3260 - t5921 * t13941 + F::cast_from(4.0_f64) * t19509 * t20183 + F::cast_from(2.0_f64) * t5739 * t5745 * t20154 * t1639 * t520 + F::cast_from(2.0_f64) * t5739 * t5745 * t6419 * t4459 * t520 - t1772 * t1773 * t522 * t71809 + F::cast_from(2.0_f64) * t5739 * t5740 * t5918 * t5448 - F::cast_from(6.0_f64) * t5739 * t18490 * t21830 * t1265 + F::cast_from(2.0_f64) * t5921 * t13889;
    (t71809, t71823)
}
