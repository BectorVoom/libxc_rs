//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2186;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2187;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2188;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2189;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2190;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2191;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2192;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta628(t100882: f64, t100926: f64, t18875: f64, t94245: f64, t25759: f64, t61203: f64, t98674: f64, t98759: f64, t98651: f64, t15071: f64, t33: f64, t1940: f64, t2403: f64, t25206: f64, t25781: f64, t27158: f64, t27364: f64, t27368: f64, t27764: f64, t3351: f64, t7091: f64, t7200: f64, t7783: f64, t98635: f64, t98650: f64, t98669: f64, t98684: f64, t99537: f64, t11064: f64, t1113: f64, t27384: f64, t27799: f64, t98767: f64, t41154: f64, t98786: f64, t1711: f64, t2411: f64, t14365: f64, t1544: f64, t4343: f64, t1583: f64, t63164: f64, t1963: f64, t25440: f64, t25752: f64, t25760: f64, t25784: f64, t27382: f64, t27770: f64, t27793: f64, t27806: f64, t4541: f64, t7869: f64, t92775: f64, t92819: f64, t98637: f64, t4433: f64, t892: f64, t14749: f64, t27763: f64, t14767: f64, t2408: f64, t14468: f64, t61102: f64, t61182: f64, t25436: f64, t25445: f64, t25763: f64, t25778: f64, t27773: f64, t27800: f64, t7087: f64, t7207: f64, t7862: f64, t98719: f64, t98722: f64, t98784: f64, t99555: f64, t98779: f64, t2394: f64, t2430: f64, t27375: f64, t61155: f64, t2832: f64, t4537: f64, t25767: f64, t27777: f64, t27802: f64, t27810: f64, t27817: f64, t51780: f64, t7863: f64, t99542: f64, t28182: f64, t7235: f64, t13392: f64, t603: f64, t13396: f64, t13405: f64, t4237: f64, t644: f64, t77: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t100927, t100973) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2186(t100882, t100926, t18875, t94245, t25759, t61203, t98674, t98759, t98651, t15071, t33, t1940, t2403, t25206, t25781, t27158, t27364, t27368, t27764, t3351, t7091, t7200, t7783, t98635, t98650, t98669, t98684, t99537);
        let (t100975, t100978, t100982, t100988, t100993) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2187(t11064, t1113, t27384, t27799, t98767, t33, t41154, t98786, t1711, t2411, t14365, t1544, t3351);
        let t101021 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2188(t1113, t4343, t1583, t3351, t27799, t63164, t100975, t100978, t100982, t100988, t100993, t1940, t1963, t2403, t25206, t25440, t25752, t25760, t25784, t27368, t27382, t27770, t27793, t27806, t4541, t7091, t7783, t7869, t92775, t92819, t98637);
        let (t101029, t101032, t101035, t101040, t101051, t101055) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2189(t1113, t4433, t892, t14749, t27763, t14767, t1711, t2408, t14468, t33, t25759, t61102);
        let t101064 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2190(t25759, t61182, t101029, t101032, t101035, t101040, t101051, t101055, t1711, t1940, t1963, t2403, t25206, t25436, t25445, t25763, t25778, t27158, t27773, t27800, t7087, t7207, t7783, t7862, t98719, t98722, t98784, t99555);
        let t101105 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2191(t27799, t98779, t1711, t2394, t2430, t27375, t94245, t61155, t2832, t1113, t4537, t1940, t1963, t2403, t25206, t25440, t25767, t27364, t27382, t27777, t27802, t27810, t27817, t4541, t51780, t7087, t7091, t7783, t7863, t99542);
        let (t101107, t101124, t101129, t101132, t101139, t101156) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2192(t100973, t101021, t101064, t101105, t28182, t7235, t13392, t603, t13396, t13405, t4237, t644, t77);
    (t100927, t101107, t101124, t101129, t101132, t101139, t101156)
}
