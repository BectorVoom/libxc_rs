//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1241;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1242;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1243;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1244;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta283(t7637: f64, t8201: f64, t1811: f64, t2148: f64, t1828: f64, t2142: f64, t7652: f64, t1287: f64, t1794: f64, t7660: f64, t2150: f64, t473: f64, t8190: f64, t1770: f64, t1775: f64, t1829: f64, t2144: f64, t2149: f64, t2152: f64, t460: f64, t7602: f64, t7632: f64, t7636: f64, t7643: f64, t7651: f64, t7659: f64, t8192: f64, t8198: f64, t33: f64, t265: f64, t502: f64, t1300: f64, t1832: f64, t198: f64, t336: f64, t5023: f64, t7673: f64, t7855: f64, t1469: f64, t2159: f64, t57: f64, t7876: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t8166: f64, t1518: f64, t7586: f64, t7888: f64, t7891: f64, t7893: f64, t8152: f64, t118: f64, t1502: f64, t1519: f64, t1843: f64, t1911: f64, t2127: f64, t2163: f64, t2165: f64, t508: f64, t569: f64, t651: f64, t7731: f64, t7734: f64, t7737: f64, t7744: f64, t7899: f64, t7903: f64, t7936: f64, t7938: f64, t8158: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8202, t8205, t8208) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1241(t7637, t8201, t1811, t2148, t1828, t2142);
        let (t8209, t8213, t8217, t8220) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1242(t7652, t8208, t1287, t1794, t7660, t2150, t473, t8190, t1770, t1775, t1829, t2144, t2149, t2152, t460, t7602, t7632, t7636, t7643, t7651, t7659, t8192, t8198, t8202, t8205);
        let (t8227, t8232) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1243(t33, t265, t502, t1300, t1832, t198, t336, t5023, t7673, t7855, t8220, t1469, t2159, t57, t7876, dens_threshold, rho1, zeta_threshold);
        let t8233 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1244(t8166, t8232);
        let (t8237, t8240) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1245(t1518, t7586, t7888, t7891, t7893, t8152, t118, t1502, t1519, t1843, t1911, t2127, t2163, t2165, t508, t569, t651, t7731, t7734, t7737, t7744, t7899, t7903, t7936, t7938, t8158, t8233);
    (t8202, t8205, t8208, t8209, t8213, t8217, t8220, t8227, t8233, t8237, t8240)
}
