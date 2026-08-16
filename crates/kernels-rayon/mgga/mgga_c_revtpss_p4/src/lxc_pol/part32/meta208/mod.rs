//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta208 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk903;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk904;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk905;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk906;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk907;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta208(t33: f64, t265: f64, t502: f64, t4560: f64, t5508: f64, t1113: f64, t1304: f64, t1469: f64, t1587: f64, t1711: f64, t1837: f64, t4186: f64, t4568: f64, t504: f64, t57: f64, t606: f64, t895: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t5035: f64, t670: f64, t93: f64, t1312: f64, t1518: f64, t2322: f64, t4246: f64, t4248: f64, t4292: f64, t1450: f64, t1907: f64, t198: f64, t530: f64, t1868: f64, t566: f64, t532: f64, t4147: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5509, t5516) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk903(t33, t265, t502, t4560, t5508, t1113, t1304, t1469, t1587, t1711, t1837, t4186, t4568, t504, t57, t606, t895, dens_threshold, rho1, zeta_threshold);
        let t5517 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk904(t5035, t5516);
        let t5523 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk905(t670, t93);
        let (t5528, t5532, t5536) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk906(t1312, t1518, t2322, t4246, t4248, t4292, t5523, t670, t1450, t1907, t198, t530);
        let (t5537, t5541, t5542) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk907(t1868, t566, t198, t532, t1907, t4147);
    (t5509, t5517, t5523, t5528, t5532, t5536, t5537, t5541, t5542)
}
