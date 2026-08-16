//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1473/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1473(t1174: f64, t1177: f64, t1196: f64, t1227: f64, t1735: f64, t18321: f64, t21758: f64, t22129: f64, t22133: f64, t22137: f64, t22197: f64, t22258: f64, t3560: f64, t3577: f64, t45128: f64, t4582: f64, t4889: f64, t4987: f64, t5024: f64, t6184: f64, t6188: f64, t73076: f64, t75847: f64, t75912: f64, t77621: f64, t78043: f64, t78047: f64, t974: f64) -> f64 {
    let t79320 = -5.0_f64 / 216.0_f64 * t5024 * t22197 + t5024 * t22258 / 36.0_f64 + 5.0_f64 / 3456.0_f64 * t1227 * t4582 * t4987 * t77621 - t1174 * t1177 * t78047 / 36.0_f64 - t1174 * t1177 * t78043 / 8.0_f64 - 11.0_f64 / 54.0_f64 * t18321 * t6184 - 11.0_f64 / 27.0_f64 * t18321 * t6188 + t1174 * t974 * t3560 * t75847 / 72.0_f64 - 8.0_f64 / 27.0_f64 * t4889 * t22137 + t4889 * t22129 / 27.0_f64 + 2.0_f64 / 9.0_f64 * t4889 * t22133 - t1174 * t974 * t1196 * t75912 / 288.0_f64 - 4.0_f64 / 81.0_f64 * t73076 - 5.0_f64 / 1296.0_f64 * t3577 * t45128 * t1735 * t21758;
    t79320
}
