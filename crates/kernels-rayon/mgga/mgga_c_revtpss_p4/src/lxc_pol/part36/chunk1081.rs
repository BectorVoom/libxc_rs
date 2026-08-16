//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1081/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1081(t1715: f64, t21093: f64, t1042: f64, t1774: f64, t5819: f64, t5268: f64, t6573: f64, t482: f64, t371: f64, t372: f64, t12988: f64, t17308: f64, t17362: f64, t17417: f64, t17525: f64, t1791: f64, t1797: f64, t20820: f64, t20974: f64, t21001: f64, t21063: f64, t3711: f64, t5293: f64, t5323: f64, t5327: f64, t5384: f64, t6611: f64, t6625: f64, t6631: f64, t6647: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24604 = t21093 * t1715;
    let t24605 = t1042 * t24604;
    let t24610 = t5819 * t1774;
    let t24611 = t5268 * t24610;
    let t24612 = t1042 * t24611;
    let t24616 = t6573 * t1774;
    let t24617 = t482 * t24616;
    let t24619 = t371 * t372 * t24617;
    let t24622 = -0.64311027177104605458e-3_f64 * t5327 * t6647 + 0.12862205435420921092e-2_f64 * t17308 * t6611 + 0.68598428988911579154e-2_f64 * t21063 * t1791 + 0.34299214494455789577e-2_f64 * t5323 * t6647 - 0.28582678745379824648e-3_f64 * t20974 + 0.64311027177104605458e-3_f64 * t20820 * t1797 - 0.34299214494455789577e-2_f64 * t5293 * t6625 - 0.68598428988911579154e-2_f64 * t17525 * t6631 - 0.85748036236139473944e-3_f64 * t5384 * t24605 - 0.14291339372689912324e-3_f64 * t17362 + 0.30488190661738479624e-2_f64 * t21001 + 0.85748036236139473944e-3_f64 * t3711 * t24612 + 0.95275595817932748825e-4_f64 * t17417 - 0.12862205435420921092e-2_f64 * t12988 * t24619;
    (t24605, t24610, t24612, t24616, t24619, t24622)
}
