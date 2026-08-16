//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1113/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1113(t4781: f64, t4785: f64, t3313: f64, t11277: f64, t5988: f64, t1117: f64, t11275: f64, t3411: f64, t6106: f64, t1157: f64, t6105: f64, t1164: f64) -> (f64, f64, f64, f64) {
    let t18262 = t4785 * t4781;
    let t18264 = 0.32163958997385070134e2_f64 * t3313 * t18262;
    let t18265 = t5988 * t11277;
    let t18266 = t18265 * t1117;
    let t18268 = 0.51726012919273400301e3_f64 * t11275 * t18266;
    let t18270 = 0.17315859105681463759e2_f64 * t3411 * t6106;
    let t18271 = t6105 * t1157;
    let t18273 = 0.35089341735807877242e1_f64 * t1164 * t18271;
    (t18264, t18268, t18270, t18273)
}
