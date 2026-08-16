//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1274/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1274(t11970: f64, t1973: f64, t1058: f64, t25554: f64, t3201: f64, t7126: f64, t25561: f64, t7114: f64, t25566: f64, t1020: f64, t11663: f64, t11680: f64, t11698: f64, t11903: f64, t11960: f64, t1971: f64, t1972: f64, t25517: f64, t25553: f64, t25569: f64, t27493: f64, t3184: f64, t3196: f64, t351: f64, t375: f64, t7125: f64) -> f64 {
    let t93611 = 0.1270341277572436651e-3_f64 * t1973 * t11970;
    let t93616 = t25554 * t1058;
    let t93618 = t7126 * t3201;
    let t93620 = t25561 * t1058;
    let t93622 = t7114 * t3201;
    let t93627 = t25566 * t1058;
    let t93641 = 0.14291339372689912324e-2_f64 * t25569 * t3184 + t93611 - 0.10620053080505570402e0_f64 * t351 * t1971 * t11960 * t375 + 0.28963781128651555642e-1_f64 * t93616 + 0.15244095330869239812e-2_f64 * t93618 - 0.91464571985215438873e-2_f64 * t93620 - 0.28582678745379824648e-3_f64 * t93622 - 0.68598428988911579154e-2_f64 * t3196 * t7125 * t375 + 0.85748036236139473944e-3_f64 * t93627 + 0.43445671692977333464e-1_f64 * t1020 * t25553 * t375 + 0.42874018118069736972e-3_f64 * t11903 * t1972 * t375 + 0.85748036236139473944e-3_f64 * t25517 * t11680 + 0.17149607247227894789e-2_f64 * t27493 * t11663 + 0.85748036236139473944e-3_f64 * t25517 * t11698;
    t93641
}
