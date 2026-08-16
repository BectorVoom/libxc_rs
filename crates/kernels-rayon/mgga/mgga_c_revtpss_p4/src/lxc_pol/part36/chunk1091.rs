//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1091/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1091(t24768: f64, t24769: f64, t1250: f64, t482: f64, t1042: f64, t1247: f64, t1261: f64, t12866: f64, t12910: f64, t17396: f64, t17401: f64, t17505: f64, t1797: f64, t21107: f64, t21252: f64, t21255: f64, t24726: f64, t24731: f64, t24736: f64, t24741: f64, t24744: f64, t24753: f64, t24759: f64, t3711: f64, t3718: f64, t5331: f64, t5340: f64, t6619: f64, t6690: f64) -> (f64, f64, f64) {
    let t24770 = t24768 + t24769;
    let t24772 = t482 * t24770 * t1250;
    let t24773 = t1042 * t24772;
    let t24778 = -t21252 / 288.0_f64 - t21255 / 144.0_f64 - 0.85748036236139473944e-3_f64 * t1261 * t24726 + 0.12862205435420921092e-2_f64 * t5340 * t24731 - 0.64311027177104605458e-3_f64 * t5331 * t24736 + 0.12862205435420921092e-2_f64 * t12910 * t24741 + 0.85748036236139473944e-3_f64 * t12866 * t24744 + 0.68598428988911579154e-2_f64 * t17396 * t6690 - 0.12862205435420921092e-2_f64 * t17401 * t6690 - 0.64311027177104605458e-3_f64 * t3718 * t24753 - 0.68598428988911579154e-2_f64 * t21107 * t1797 + 0.42874018118069736972e-3_f64 * t3711 * t24759 + 0.21437009059034868486e-3_f64 * t1247 * t24773 - 0.45732285992607719436e-2_f64 * t17505 * t6619;
    (t24770, t24773, t24778)
}
