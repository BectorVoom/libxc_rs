//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta207 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk961;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk962;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk963;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk964;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk965;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk966;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk967;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk968;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta207<F: Float>(t1277: F, t5497: F, t1204: F, t1210: F, t1215: F, t1271: F, t1274: F, t1295: F, t1770: F, t1775: F, t1813: F, t1829: F, t3556: F, t3561: F, t3567: F, t3572: F, t3732: F, t460: F, t495: F, t5216: F, t5220: F, t5225: F, t5231: F, t5237: F, t5246: F, t5251: F, t5414: F, t5417: F, t5423: F, t5429: F, t1832: F, t3801: F, t1298: F, t1300: F, t198: F, t336: F, t5023: F, t5062: F, t5065: F, t5067: F, t5070: F, t5107: F, t5111: F, t5189: F, t5191: F, t5194: F, t5196: F, t5200: F, t5204: F, t5209: F, t33: F, t265: F, t502: F, t4560: F, t1113: F, t1304: F, t1469: F, t1587: F, t1711: F, t1837: F, t4186: F, t4568: F, t504: F, t57: F, t606: F, t895: F, dens_threshold: F, rho1: F, zeta_threshold: F, t5035: F, t670: F, t93: F, t1312: F, t1518: F, t2322: F, t4246: F, t4248: F, t4292: F, t1450: F, t1907: F, t530: F, t1868: F, t566: F, t532: F, t4147: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t5498 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk961::<F>(t1277, t5497);
        let t5501 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk962::<F>(t1204, t1210, t1215, t1271, t1274, t1295, t1770, t1775, t1813, t1829, t3556, t3561, t3567, t3572, t3732, t460, t495, t5216, t5220, t5225, t5231, t5237, t5246, t5251, t5414, t5417, t5423, t5429, t5498);
        let (t5505, t5508) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk963::<F>(t1832, t3801, t1298, t1300, t198, t336, t5023, t5062, t5065, t5067, t5070, t5107, t5111, t5189, t5191, t5194, t5196, t5200, t5204, t5209, t5501);
        let (t5509, t5516) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk964::<F>(t33, t265, t502, t4560, t5508, t1113, t1304, t1469, t1587, t1711, t1837, t4186, t4568, t504, t57, t606, t895, dens_threshold, rho1, zeta_threshold);
        let t5517 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk965::<F>(t5035, t5516);
        let t5523 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk966::<F>(t670, t93);
        let (t5528, t5532, t5536) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk967::<F>(t1312, t1518, t2322, t4246, t4248, t4292, t5523, t670, t1450, t1907, t198, t530);
        let (t5537, t5541, t5542) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk968::<F>(t1868, t566, t198, t532, t1907, t4147);
    (t5498, t5501, t5505, t5509, t5517, t5523, t5528, t5532, t5536, t5537, t5541, t5542)
}
