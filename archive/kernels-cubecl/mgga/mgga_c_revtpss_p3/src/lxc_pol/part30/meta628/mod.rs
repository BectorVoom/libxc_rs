//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2186;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2187;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2188;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2189;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2190;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2191;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2192;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta628<F: Float>(t100882: F, t100926: F, t18875: F, t94245: F, t25759: F, t61203: F, t98674: F, t98759: F, t98651: F, t15071: F, t33: F, t1940: F, t2403: F, t25206: F, t25781: F, t27158: F, t27364: F, t27368: F, t27764: F, t3351: F, t7091: F, t7200: F, t7783: F, t98635: F, t98650: F, t98669: F, t98684: F, t99537: F, t11064: F, t1113: F, t27384: F, t27799: F, t98767: F, t41154: F, t98786: F, t1711: F, t2411: F, t14365: F, t1544: F, t4343: F, t1583: F, t63164: F, t1963: F, t25440: F, t25752: F, t25760: F, t25784: F, t27382: F, t27770: F, t27793: F, t27806: F, t4541: F, t7869: F, t92775: F, t92819: F, t98637: F, t4433: F, t892: F, t14749: F, t27763: F, t14767: F, t2408: F, t14468: F, t61102: F, t61182: F, t25436: F, t25445: F, t25763: F, t25778: F, t27773: F, t27800: F, t7087: F, t7207: F, t7862: F, t98719: F, t98722: F, t98784: F, t99555: F, t98779: F, t2394: F, t2430: F, t27375: F, t61155: F, t2832: F, t4537: F, t25767: F, t27777: F, t27802: F, t27810: F, t27817: F, t51780: F, t7863: F, t99542: F, t28182: F, t7235: F, t13392: F, t603: F, t13396: F, t13405: F, t4237: F, t644: F, t77: F) -> (F, F, F, F, F, F, F) {
        let (t100927, t100973) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2186::<F>(t100882, t100926, t18875, t94245, t25759, t61203, t98674, t98759, t98651, t15071, t33, t1940, t2403, t25206, t25781, t27158, t27364, t27368, t27764, t3351, t7091, t7200, t7783, t98635, t98650, t98669, t98684, t99537);
        let (t100975, t100978, t100982, t100988, t100993) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2187::<F>(t11064, t1113, t27384, t27799, t98767, t33, t41154, t98786, t1711, t2411, t14365, t1544, t3351);
        let t101021 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2188::<F>(t1113, t4343, t1583, t3351, t27799, t63164, t100975, t100978, t100982, t100988, t100993, t1940, t1963, t2403, t25206, t25440, t25752, t25760, t25784, t27368, t27382, t27770, t27793, t27806, t4541, t7091, t7783, t7869, t92775, t92819, t98637);
        let (t101029, t101032, t101035, t101040, t101051, t101055) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2189::<F>(t1113, t4433, t892, t14749, t27763, t14767, t1711, t2408, t14468, t33, t25759, t61102);
        let t101064 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2190::<F>(t25759, t61182, t101029, t101032, t101035, t101040, t101051, t101055, t1711, t1940, t1963, t2403, t25206, t25436, t25445, t25763, t25778, t27158, t27773, t27800, t7087, t7207, t7783, t7862, t98719, t98722, t98784, t99555);
        let t101105 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2191::<F>(t27799, t98779, t1711, t2394, t2430, t27375, t94245, t61155, t2832, t1113, t4537, t1940, t1963, t2403, t25206, t25440, t25767, t27364, t27382, t27777, t27802, t27810, t27817, t4541, t51780, t7087, t7091, t7783, t7863, t99542);
        let (t101107, t101124, t101129, t101132, t101139, t101156) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2192::<F>(t100973, t101021, t101064, t101105, t28182, t7235, t13392, t603, t13396, t13405, t4237, t644, t77);
    (t100927, t101107, t101124, t101129, t101132, t101139, t101156)
}
