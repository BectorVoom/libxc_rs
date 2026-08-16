//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta72 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk502;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk503;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk504;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk505;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk506;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk507;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta72<F: Float>(t565: F, t1319: F, t1322: F, t1332: F, t1334: F, t1336: F, t1339: F, t1342: F, t1343: F, t1353: F, t1448: F, t198: F, t532: F, t679: F, t704: F, t118: F, t1310: F, t1315: F, t508: F, t511: F, t569: F, t649: F, t651: F, t671: F, t3: F, t571: F, t117: F, t670: F, t572: F, t573: F, t578: F, t582: F, t586: F, t590: F, t594: F, t598: F, t4: F, t604: F, param_d: F, t30: F, t33: F, zeta_threshold: F, t36: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1450 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk502::<F>(t565);
        let t1453 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk503::<F>(t1319, t1322, t1332, t1334, t1336, t1339, t1342, t1343, t1353, t1448, t1450, t198, t532, t679, t704);
        let (t1455, t1456, t1458) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk504::<F>(t118, t1310, t1315, t1453, t508, t511, t569, t649, t651, t671, t3, t571);
        let (t1459, t1461, t1464, t1466, t1468) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk505::<F>(t1455, t117, t670, t572, t573, t578, t582, t586, t590, t594, t598, t4, t604, param_d);
        let t1469 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk506::<F>(t30, t33, t1468, zeta_threshold);
        let t1470 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk507::<F>(t1469, t36);
    (t1450, t1453, t1455, t1456, t1458, t1459, t1461, t1464, t1466, t1468, t1469, t1470)
}
