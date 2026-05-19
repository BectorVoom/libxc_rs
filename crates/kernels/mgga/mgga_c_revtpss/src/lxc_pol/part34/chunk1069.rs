//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1069/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1069<F: Float>(t24768: F, t24769: F, t1250: F, t482: F, t1042: F, t1247: F, t1261: F, t12866: F, t12910: F, t17396: F, t17401: F, t17505: F, t1797: F, t21107: F, t21252: F, t21255: F, t24726: F, t24731: F, t24736: F, t24741: F, t24744: F, t24753: F, t24759: F, t3711: F, t3718: F, t5331: F, t5340: F, t6619: F, t6690: F) -> (F, F) {
    let t24770 = t24768 + t24769;
    let t24772 = t482 * t24770 * t1250;
    let t24773 = t1042 * t24772;
    let t24778 = -t21252 / F::new(288.0) - t21255 / F::new(144.0) - F::cast_from(0.85748036236139473944e-3_f64) * t1261 * t24726 + F::cast_from(0.12862205435420921092e-2_f64) * t5340 * t24731 - F::cast_from(0.64311027177104605458e-3_f64) * t5331 * t24736 + F::cast_from(0.12862205435420921092e-2_f64) * t12910 * t24741 + F::cast_from(0.85748036236139473944e-3_f64) * t12866 * t24744 + F::cast_from(0.68598428988911579154e-2_f64) * t17396 * t6690 - F::cast_from(0.12862205435420921092e-2_f64) * t17401 * t6690 - F::cast_from(0.64311027177104605458e-3_f64) * t3718 * t24753 - F::cast_from(0.68598428988911579154e-2_f64) * t21107 * t1797 + F::cast_from(0.42874018118069736972e-3_f64) * t3711 * t24759 + F::cast_from(0.21437009059034868486e-3_f64) * t1247 * t24773 - F::cast_from(0.45732285992607719436e-2_f64) * t17505 * t6619;
    (t24770, t24778)
}
