//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1541;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1542;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1543;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1544;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1545;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta411(t15127: f64, t2852: f64, t4186: f64, t606: f64, t2850: f64, t128: f64, t2258: f64, t4573: f64, t11144: f64, t1469: f64, t2251: f64, t11142: f64, t2857: f64, t904: f64, t4578: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15128, t15130, t15132) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1541(t15127, t2852, t4186, t606, t2850, t128);
        let (t15135, t15137) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1542(t2258, t4573, t2850, t128);
        let (t15140, t15142) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1543(t11144, t1469, t2251, t11142, t128);
        let (t15145, t15147) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1544(t2857, t4186, t606, t904, t128);
        let (t15149, t15151) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1545(t2258, t4578, t904, t128);
    (t15128, t15130, t15132, t15135, t15137, t15140, t15142, t15145, t15147, t15149, t15151)
}
