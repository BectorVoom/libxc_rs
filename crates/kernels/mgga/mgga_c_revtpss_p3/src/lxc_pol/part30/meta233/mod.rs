//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta233 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1060;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1061;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1062;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1063;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta233<F: Float>(t1196: F, t5202: F, t1756: F, t3520: F, t1187: F, t3523: F, t3358: F, t3546: F, t5044: F, t5049: F, t5054: F, t5058: F, t459: F, t1208: F, t1769: F, t487: F, t1770: F, t1214: F, t1774: F, t1211: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5204, t5205, t5206, t5207, t5209, t5215) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1060::<F>(t1196, t5202, t1756, t3520, t1187, t3523, t3358, t3546, t5044, t5049, t5054, t5058);
        let t5216 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1061::<F>(t459, t5215);
        let t5219 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1062::<F>(t1208, t1769);
        let (t5220, t5225, t5230) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1063::<F>(t487, t5219, t1770, t1214, t1774);
        let t5231 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1064::<F>(t1211, t5230);
    (t5204, t5205, t5206, t5207, t5209, t5215, t5216, t5219, t5220, t5225, t5230, t5231)
}
