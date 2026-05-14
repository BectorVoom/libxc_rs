//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 765/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk765<F: Float>(t1797: F, t570: F, t1784: F, t1886: F, t2001: F, t1881: F, t1844: F, t599: F, t1181: F, t2068: F, t336: F, t5630: F, t8040: F, t9476: F, t8129: F, t8453: F, t8459: F, t8478: F, t8492: F, t8494: F, t8507: F, t8509: F, t8527: F, t8529: F, t8531: F, t8533: F, t8546: F, t8556: F, t8558: F, t8572: F, t8574: F, t8578: F, t9522: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9749 = t570 * t1797;
    let t9751 = t570 * t1784;
    let t9753 = t2001 * t1886;
    let t9755 = t2001 * t1881;
    let t9757 = t599 * t1844;
    let t9758 = t1181 * t9757;
    let t9759 = t2068 * t9758;
    let t9761 = t336 * t5630;
    let t9762 = t570 * t9761;
    let t9826 = t8040 * t9476;
    let t9853 = 0.17149607247227894789e-2 * t8453 + t8129 - 0.31448092289604152068e-2 * t8459 + 0.12579236915841660828e-2 * t8478 + 0.12579236915841660828e-2 * t8492 - 0.85748036236139473944e-3 * t8494 - 0.57165357490759649296e-3 * t8507 + 0.31448092289604152068e-2 * t8509 + 0.17149607247227894789e-2 * t9522 - 0.28582678745379824648e-3 * t8527 + 0.62896184579208304138e-3 * t8529 + 0.25724410870841842184e-2 * t8531 - 0.21437009059034868486e-2 * t8533 - 0.14291339372689912324e-2 * t8546 + 0.20965394859736101379e-2 * t8556 - 0.12579236915841660828e-2 * t8558 - 0.83861579438944405517e-3 * t8572 - 0.17149607247227894789e-2 * t8574 + 0.18868855373762491241e-2 * t8578;
    (t9749, t9751, t9753, t9755, t9757, t9758, t9759, t9761, t9762, t9826, t9853)
}
