//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1368/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1368(t67149: f64, t67163: f64, t67177: f64, t67191: f64, t13051: f64, t1639: f64, t1772: f64, t1773: f64, t1842: f64, t18481: f64, t18483: f64, t18496: f64, t18947: f64, t18950: f64, t18967: f64, t18972: f64, t18986: f64, t18991: f64, t18997: f64, t19507: f64, t19509: f64, t19540: f64, t19542: f64, t20171: f64, t20190: f64, t20191: f64, t20214: f64, t3326: f64, t43933: f64, t4494: f64, t4517: f64, t520: f64, t522: f64, t538: f64, t5737: f64, t5739: f64, t5745: f64, t5921: f64, t5933: f64, t6260: f64, t6419: f64, t6433: f64, t65685: f64, t65719: f64, t65783: f64, t67131: f64, param_beta: f64) -> f64 {
    let t67193 = t67149 + t67163 + t67177 + t67191;
    let t67211 = -12.0_f64 * t18483 * t20171 - 2.0_f64 * t19507 * t5933 - 4.0_f64 * t65719 * t20191 - t6260 * t18997 - t18481 * t6433 + t5739 * t5745 * t6419 * t3326 * t520 - 2.0_f64 * t5737 * t20214 + 4.0_f64 * t5921 * t13051 + 4.0_f64 * t18950 * t4494 - 2.0_f64 * t18496 * t18967 * t65783 - 4.0_f64 * t19540 * t67131 * t19542 - 4.0_f64 * t19540 * t20190 * t43933 - t1772 * t1773 * t522 * t67193 + t5739 * t5745 * t18947 * t1639 * t520 - 2.0_f64 * t18950 * t4517 - t65685 * t1842 + param_beta * t67193 * t538 + 2.0_f64 * t19509 * t18986 + t19509 * t18991 + 4.0_f64 * t19509 * t18972;
    t67211
}
