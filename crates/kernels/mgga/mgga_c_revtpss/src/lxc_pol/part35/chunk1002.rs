//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1002/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1002<F: Float>(t1715: F, t21093: F, t1042: F, t1774: F, t5819: F, t5268: F, t6573: F, t482: F, t371: F, t372: F, t12988: F, t17308: F, t17362: F, t17417: F, t17525: F, t1791: F, t1797: F, t20820: F, t20974: F, t21001: F, t21063: F, t3711: F, t5293: F, t5323: F, t5327: F, t5384: F, t6611: F, t6625: F, t6631: F, t6647: F) -> (F, F, F) {
    let t24604 = t21093 * t1715;
    let t24605 = t1042 * t24604;
    let t24610 = t5819 * t1774;
    let t24611 = t5268 * t24610;
    let t24612 = t1042 * t24611;
    let t24616 = t6573 * t1774;
    let t24617 = t482 * t24616;
    let t24619 = t371 * t372 * t24617;
    let t24622 = -F::new(0.64311027177104605458e-3) * t5327 * t6647 + F::new(0.12862205435420921092e-2) * t17308 * t6611 + F::new(0.68598428988911579154e-2) * t21063 * t1791 + F::new(0.34299214494455789577e-2) * t5323 * t6647 - F::new(0.28582678745379824648e-3) * t20974 + F::new(0.64311027177104605458e-3) * t20820 * t1797 - F::new(0.34299214494455789577e-2) * t5293 * t6625 - F::new(0.68598428988911579154e-2) * t17525 * t6631 - F::new(0.85748036236139473944e-3) * t5384 * t24605 - F::new(0.14291339372689912324e-3) * t17362 + F::new(0.30488190661738479624e-2) * t21001 + F::new(0.85748036236139473944e-3) * t3711 * t24612 + F::new(0.95275595817932748825e-4) * t17417 - F::new(0.12862205435420921092e-2) * t12988 * t24619;
    (t24610, t24616, t24622)
}
