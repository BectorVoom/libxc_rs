//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1140/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1140<F: Float>(t11916: F, t25509: F, t25569: F, t3111: F, t11722: F, t7132: F, t11727: F, t12002: F, t1971: F, t351: F, t1052: F, t3089: F, t1087: F, t11744: F, t1068: F, t11748: F, t11806: F, t11845: F, t11855: F, t11983: F, t25577: F, t27536: F, t3097: F, t3101: F, t3177: F, sigma0: F) -> (F, F) {
    let t93573 = t25509 * t11916;
    let t93579 = t25569 * t3111;
    let t93583 = t7132 * t11722;
    let t93585 = t7132 * t11727;
    let t93592 = t351 * t1971 * t12002;
    let t93595 = sigma0 * t1052;
    let t93596 = t93595 * t3089;
    let t93597 = t1087 * t93596;
    let t93602 = t7132 * t11744;
    let t93606 = -0.85748036236139473944e-3 * t93573 + 0.14291339372689912324e-2 * t7132 * t11983 + 0.1270341277572436651e-2 * t7132 * t11855 + 0.11433071498151929859e-2 * t93579 + 0.25724410870841842183e-2 * t27536 * t11806 + 0.57165357490759649295e-3 * t93583 + 0.95275595817932748825e-3 * t93585 + 0.85748036236139473944e-3 * t25569 * t3177 + 0.28582678745379824648e-3 * t7132 * t11845 + 0.28963781128651555642e-1 * t93592 * t1068 - 0.91464571985215438873e-2 * t93597 * t3097 + 0.91464571985215438873e-2 * t25577 * t3101 - 0.11433071498151929859e-2 * t93602 + 0.17149607247227894789e-2 * t7132 * t11748;
    (t93596, t93606)
}
