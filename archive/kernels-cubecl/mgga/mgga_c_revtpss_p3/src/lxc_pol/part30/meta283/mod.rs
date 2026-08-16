//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1241;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1242;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1243;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1244;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta283<F: Float>(t7637: F, t8201: F, t1811: F, t2148: F, t1828: F, t2142: F, t7652: F, t1287: F, t1794: F, t7660: F, t2150: F, t473: F, t8190: F, t1770: F, t1775: F, t1829: F, t2144: F, t2149: F, t2152: F, t460: F, t7602: F, t7632: F, t7636: F, t7643: F, t7651: F, t7659: F, t8192: F, t8198: F, t33: F, t265: F, t502: F, t1300: F, t1832: F, t198: F, t336: F, t5023: F, t7673: F, t7855: F, t1469: F, t2159: F, t57: F, t7876: F, dens_threshold: F, rho1: F, zeta_threshold: F, t8166: F, t1518: F, t7586: F, t7888: F, t7891: F, t7893: F, t8152: F, t118: F, t1502: F, t1519: F, t1843: F, t1911: F, t2127: F, t2163: F, t2165: F, t508: F, t569: F, t651: F, t7731: F, t7734: F, t7737: F, t7744: F, t7899: F, t7903: F, t7936: F, t7938: F, t8158: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t8202, t8205, t8208) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1241::<F>(t7637, t8201, t1811, t2148, t1828, t2142);
        let (t8209, t8213, t8217, t8220) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1242::<F>(t7652, t8208, t1287, t1794, t7660, t2150, t473, t8190, t1770, t1775, t1829, t2144, t2149, t2152, t460, t7602, t7632, t7636, t7643, t7651, t7659, t8192, t8198, t8202, t8205);
        let (t8227, t8232) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1243::<F>(t33, t265, t502, t1300, t1832, t198, t336, t5023, t7673, t7855, t8220, t1469, t2159, t57, t7876, dens_threshold, rho1, zeta_threshold);
        let t8233 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1244::<F>(t8166, t8232);
        let (t8237, t8240) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1245::<F>(t1518, t7586, t7888, t7891, t7893, t8152, t118, t1502, t1519, t1843, t1911, t2127, t2163, t2165, t508, t569, t651, t7731, t7734, t7737, t7744, t7899, t7903, t7936, t7938, t8158, t8233);
    (t8202, t8205, t8208, t8209, t8213, t8217, t8220, t8227, t8233, t8237, t8240)
}
