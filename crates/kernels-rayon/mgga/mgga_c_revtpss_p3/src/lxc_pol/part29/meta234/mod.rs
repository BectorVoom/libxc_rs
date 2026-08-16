//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta234 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk997;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk998;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk999;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta234(t1196: f64, t5202: f64, t1756: f64, t3520: f64, t1187: f64, t3523: f64, t3358: f64, t3546: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t459: f64, t1208: f64, t1769: f64, t487: f64, t1770: f64, t1214: f64, t1774: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5204, t5205, t5206, t5207, t5209, t5215) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk997(t1196, t5202, t1756, t3520, t1187, t3523, t3358, t3546, t5044, t5049, t5054, t5058);
        let t5216 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk998(t459, t5215);
        let t5219 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk999(t1208, t1769);
        let (t5220, t5225, t5230) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1000(t487, t5219, t1770, t1214, t1774);
    (t5204, t5205, t5206, t5207, t5209, t5215, t5216, t5219, t5220, t5225, t5230)
}
