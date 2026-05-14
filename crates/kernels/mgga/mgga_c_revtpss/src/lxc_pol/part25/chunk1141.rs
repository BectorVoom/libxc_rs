//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1141/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1141<F: Float>(t11970: F, t1973: F, t1058: F, t25554: F, t3201: F, t7126: F, t25561: F, t7114: F, t25566: F, t1020: F, t11663: F, t11680: F, t11698: F, t11903: F, t11960: F, t1971: F, t1972: F, t25517: F, t25553: F, t25569: F, t27493: F, t3184: F, t3196: F, t351: F, t375: F, t7125: F) -> (F,) {
    let t93611 = 0.1270341277572436651e-3 * t1973 * t11970;
    let t93616 = t25554 * t1058;
    let t93618 = t7126 * t3201;
    let t93620 = t25561 * t1058;
    let t93622 = t7114 * t3201;
    let t93627 = t25566 * t1058;
    let t93641 = 0.14291339372689912324e-2 * t25569 * t3184 + t93611 - 0.10620053080505570402e0 * t351 * t1971 * t11960 * t375 + 0.28963781128651555642e-1 * t93616 + 0.15244095330869239812e-2 * t93618 - 0.91464571985215438873e-2 * t93620 - 0.28582678745379824648e-3 * t93622 - 0.68598428988911579154e-2 * t3196 * t7125 * t375 + 0.85748036236139473944e-3 * t93627 + 0.43445671692977333464e-1 * t1020 * t25553 * t375 + 0.42874018118069736972e-3 * t11903 * t1972 * t375 + 0.85748036236139473944e-3 * t25517 * t11680 + 0.17149607247227894789e-2 * t27493 * t11663 + 0.85748036236139473944e-3 * t25517 * t11698;
    (t93641,)
}
