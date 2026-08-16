//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1368/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1368<F: Float>(t67149: F, t67163: F, t67177: F, t67191: F, t13051: F, t1639: F, t1772: F, t1773: F, t1842: F, t18481: F, t18483: F, t18496: F, t18947: F, t18950: F, t18967: F, t18972: F, t18986: F, t18991: F, t18997: F, t19507: F, t19509: F, t19540: F, t19542: F, t20171: F, t20190: F, t20191: F, t20214: F, t3326: F, t43933: F, t4494: F, t4517: F, t520: F, t522: F, t538: F, t5737: F, t5739: F, t5745: F, t5921: F, t5933: F, t6260: F, t6419: F, t6433: F, t65685: F, t65719: F, t65783: F, t67131: F, param_beta: F) -> F {
    let t67193 = t67149 + t67163 + t67177 + t67191;
    let t67211 = -F::cast_from(12.0_f64) * t18483 * t20171 - F::cast_from(2.0_f64) * t19507 * t5933 - F::cast_from(4.0_f64) * t65719 * t20191 - t6260 * t18997 - t18481 * t6433 + t5739 * t5745 * t6419 * t3326 * t520 - F::cast_from(2.0_f64) * t5737 * t20214 + F::cast_from(4.0_f64) * t5921 * t13051 + F::cast_from(4.0_f64) * t18950 * t4494 - F::cast_from(2.0_f64) * t18496 * t18967 * t65783 - F::cast_from(4.0_f64) * t19540 * t67131 * t19542 - F::cast_from(4.0_f64) * t19540 * t20190 * t43933 - t1772 * t1773 * t522 * t67193 + t5739 * t5745 * t18947 * t1639 * t520 - F::cast_from(2.0_f64) * t18950 * t4517 - t65685 * t1842 + param_beta * t67193 * t538 + F::cast_from(2.0_f64) * t19509 * t18986 + t19509 * t18991 + F::cast_from(4.0_f64) * t19509 * t18972;
    t67211
}
