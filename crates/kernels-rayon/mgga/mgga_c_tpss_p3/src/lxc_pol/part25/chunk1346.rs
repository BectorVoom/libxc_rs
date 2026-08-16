//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1346/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1346(t1265: f64, t1657: f64, t18483: f64, t18490: f64, t18496: f64, t18967: f64, t19507: f64, t19509: f64, t19521: f64, t19535: f64, t19540: f64, t20157: f64, t20174: f64, t20179: f64, t20187: f64, t20190: f64, t20191: f64, t20202: f64, t20214: f64, t21061: f64, t21804: f64, t21820: f64, t4494: f64, t5432: f64, t5739: f64, t5740: f64, t5918: f64, t5933: f64, t6260: f64, t6433: f64, t65871: f64, t66970: f64, t67083: f64, t69654: f64, t69676: f64, t69691: f64, t69704: f64, t69708: f64) -> f64 {
    let t71662 = 4.0_f64 * t19509 * t20179 + 2.0_f64 * t5739 * t5740 * t21804 * t1265 - t21061 * t5933 - 6.0_f64 * t18483 * t21820 - 6.0_f64 * t5739 * t18490 * t5918 * t5432 - 2.0_f64 * t6260 * t20214 - 2.0_f64 * t19507 * t6433 - 4.0_f64 * t18496 * t18967 * t69691 - 4.0_f64 * t65871 * t20174 - 4.0_f64 * t65871 * t20187 + 2.0_f64 * t69654 * t20202 + 4.0_f64 * t18496 * t20190 * t69704 - 4.0_f64 * t19540 * t20190 * t69708 - 4.0_f64 * t18496 * t66970 * t19535 - 4.0_f64 * t69654 * t20191 - 4.0_f64 * t18496 * t66970 * t19521 - 4.0_f64 * t18496 * t18967 * t69676 - 2.0_f64 * t67083 * t1657 + 4.0_f64 * t20157 * t4494;
    t71662
}
