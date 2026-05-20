//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta411 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1541;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1542;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1543;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1544;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1545;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta411<F: Float>(t15127: F, t2852: F, t4186: F, t606: F, t2850: F, t128: F, t2258: F, t4573: F, t11144: F, t1469: F, t2251: F, t11142: F, t2857: F, t904: F, t4578: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15128, t15130, t15132) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1541::<F>(t15127, t2852, t4186, t606, t2850, t128);
        let (t15135, t15137) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1542::<F>(t2258, t4573, t2850, t128);
        let (t15140, t15142) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1543::<F>(t11144, t1469, t2251, t11142, t128);
        let (t15145, t15147) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1544::<F>(t2857, t4186, t606, t904, t128);
        let (t15149, t15151) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1545::<F>(t2258, t4578, t904, t128);
    (t15128, t15130, t15132, t15135, t15137, t15140, t15142, t15145, t15147, t15149, t15151)
}
