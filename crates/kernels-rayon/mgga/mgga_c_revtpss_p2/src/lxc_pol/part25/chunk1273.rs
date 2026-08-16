//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1273/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1273(t1087: f64, t93596: f64, t11744: f64, t7132: f64, t1068: f64, t11748: f64, t11806: f64, t11845: f64, t11855: f64, t11983: f64, t25569: f64, t25577: f64, t27536: f64, t3097: f64, t3101: f64, t3177: f64, t93573: f64, t93579: f64, t93583: f64, t93585: f64, t93592: f64) -> f64 {
    let t93597 = t1087 * t93596;
    let t93602 = t7132 * t11744;
    let t93606 = -0.85748036236139473944e-3_f64 * t93573 + 0.14291339372689912324e-2_f64 * t7132 * t11983 + 0.1270341277572436651e-2_f64 * t7132 * t11855 + 0.11433071498151929859e-2_f64 * t93579 + 0.25724410870841842183e-2_f64 * t27536 * t11806 + 0.57165357490759649295e-3_f64 * t93583 + 0.95275595817932748825e-3_f64 * t93585 + 0.85748036236139473944e-3_f64 * t25569 * t3177 + 0.28582678745379824648e-3_f64 * t7132 * t11845 + 0.28963781128651555642e-1_f64 * t93592 * t1068 - 0.91464571985215438873e-2_f64 * t93597 * t3097 + 0.91464571985215438873e-2_f64 * t25577 * t3101 - 0.11433071498151929859e-2_f64 * t93602 + 0.17149607247227894789e-2_f64 * t7132 * t11748;
    t93606
}
