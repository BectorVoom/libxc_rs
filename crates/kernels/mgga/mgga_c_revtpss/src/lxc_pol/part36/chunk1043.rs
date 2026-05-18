//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1043/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1043<F: Float>(t23598: F, t373: F, t371: F, t372: F, t1651: F, t6244: F, t1011: F, t1025: F, t11859: F, t11875: F, t11941: F, t15671: F, t15926: F, t16220: F, t1665: F, t19773: F, t20005: F, t20017: F, t20021: F, t20025: F, t20030: F, t20034: F, t20051: F, t20055: F, t23994: F, t23999: F, t24009: F, t24013: F, t24017: F, t3115: F, t4858: F, t6273: F, t6278: F, t6339: F) -> (F, F) {
    let t24022 = t373 * t23598;
    let t24024 = t371 * t372 * t24022;
    let t24031 = t6244 * t1651;
    let t24032 = t373 * t24031;
    let t24034 = t371 * t372 * t24032;
    let t24040 = F::new(0.57165357490759649295e-3) * t20005 - F::new(0.12862205435420921092e-2) * t15926 * t6273 - F::new(0.64311027177104605458e-3) * t3115 * t23994 - F::new(0.64311027177104605458e-3) * t3115 * t23999 + F::new(0.85748036236139473944e-3) * t20017 - F::new(0.42874018118069736972e-3) * t20021 - F::new(0.85748036236139473944e-3) * t20025 + F::new(0.85748036236139473944e-3) * t20030 + F::new(0.85748036236139473944e-3) * t20034 - F::new(0.12862205435420921092e-2) * t11859 * t24009 + F::new(0.64311027177104605458e-3) * t11875 * t24013 + t1011 * t24017 / F::new(48.0) - F::new(0.64311027177104605458e-3) * t4858 * t6278 - F::new(0.21437009059034868486e-3) * t1025 * t24024 - F::new(0.64311027177104605458e-3) * t19773 * t1665 + F::new(0.12862205435420921092e-2) * t15671 * t6339 - F::new(0.12862205435420921092e-2) * t11941 * t24034 + F::new(0.47637797908966374413e-3) * t20051 + F::new(0.28582678745379824648e-3) * t20055 - t16220 / F::new(432.0);
    (t24031, t24040)
}
