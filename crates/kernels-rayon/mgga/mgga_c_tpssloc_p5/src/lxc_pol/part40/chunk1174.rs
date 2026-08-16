//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1174/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1174(t1155: f64, t6085: f64, t3403: f64, t6084: f64, t4857: f64, t4861: f64, t11285: f64, t6068: f64, t11310: f64, t11365: f64, t15126: f64, t15136: f64, t15146: f64, t15207: f64, t18247: f64, t18603: f64, t18606: f64, t18609: f64, t3376: f64, t3401: f64, t4802: f64, t4824: f64, t4840: f64, t4862: f64) -> f64 {
    let t18612 = t6085 * t1155;
    let t18615 = t6084 * t3403;
    let t18616 = t18615 * t1155;
    let t18619 = t4861 * t4857;
    let t18622 = t6068 * t11285;
    let t18623 = t18622 * t1155;
    let t18630 = -0.23392894490538584828e1_f64 * t15136 * t4840 + 0.34631718211362927517e2_f64 * t15126 * t4862 + 0.35089341735807877242e1_f64 * t3401 * t18603 - 0.23392894490538584828e1_f64 * t3376 * t18606 - 0.10389515463408878255e3_f64 * t11365 * t18609 - 0.11696447245269292414e1_f64 * t3376 * t18612 + 0.17315859105681463759e2_f64 * t3401 * t18616 + 0.34631718211362927518e2_f64 * t3401 * t18619 + 0.10254018858216406658e4_f64 * t11310 * t18623 + t18247 - 4.0_f64 * t15207 * t4802 + 0.64327917994770140268e2_f64 * t15146 * t4824;
    t18630
}
