//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1723;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1724;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1725;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1726;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1727;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta453(t13142: f64, t17708: f64, t3601: f64, t3603: f64, t17710: f64, t3720: f64, t13127: f64, t471: f64, t17730: f64, t5046: f64, t12787: f64, t1260: f64, t5261: f64, t3647: f64, t5378: f64, t247: f64, t3634: f64, t5056: f64, t1261: f64, t1266: f64, t17721: f64, t17724: f64, t17729: f64, t17732: f64, t17736: f64, t17739: f64, t17744: f64, t3718: f64, t16756: f64, t5333: f64, t3588: f64, t5332: f64, t12916: f64, t5334: f64, t5331: f64, t1778: f64, t3682: f64, t1774: f64, t3617: f64, t3363: f64, t1042: f64, t372: f64, t5268: f64, t17695: f64, t13086: f64, t13090: f64, t13092: f64, t17693: f64, t3640: f64, t3644: f64, t3711: f64, t5381: f64, t1247: f64, t12774: f64, t12866: f64, t12907: f64, t12918: f64, t12942: f64, t12949: f64, t12960: f64, t17199: f64, t17204: f64, t17211: f64, t17214: f64, t17219: f64, t17222: f64, t17227: f64, t17268: f64, t17299: f64, t17358: f64, t17399: f64, t17432: f64, t17470: f64, t17493: f64, t17502: f64, t17505: f64, t17509: f64, t17515: f64, t17561: f64, t17587: f64, t17614: f64, t17665: f64, t17718: f64, t3591: f64, t3701: f64, t3714: f64, t5270: f64, t5274: f64, t5373: f64, t5384: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17747, t17750, t17753, t17756, t17760, t17763) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1723(t13142, t17708, t3601, t3603, t17710, t3720, t13127, t471, t17730, t5046, t12787, t1260, t5261);
        let (t17769, t17772) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1724(t3647, t5378, t247, t3634, t5056, t1261, t1266, t17721, t17724, t17729, t17732, t17736, t17739, t17744, t17747, t17750, t17753, t17756, t17760, t17763, t3718);
        let (t17781, t17786, t17789, t17791, t17792, t17794) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1725(t16756, t5333, t3720, t3588, t471, t5332, t12916, t5334, t5331, t1778, t3682, t1774, t3617);
        let (t17796, t17800, t17803) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1726(t17794, t3363, t1042, t372, t5268, t17695, t13086, t13090, t13092, t17693, t17781, t17786, t17791, t17792, t3640, t3644, t3711, t5331, t5381);
        let t17807 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1727(t1247, t1261, t12774, t12866, t12907, t12918, t12942, t12949, t12960, t17199, t17204, t17211, t17214, t17219, t17222, t17227, t17268, t17299, t17358, t17399, t17432, t17470, t17493, t17502, t17505, t17509, t17515, t17561, t17587, t17614, t17665, t17718, t17772, t17803, t3591, t3647, t3701, t3711, t3714, t5270, t5274, t5373, t5384);
    (t17750, t17756, t17760, t17769, t17781, t17786, t17789, t17796, t17800, t17807)
}
