//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta140 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk935;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk936;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk937;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk938;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk939;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk940;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk941;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk942;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk943;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta140<F: Float>(t2257: F, t1941: F, t268: F, t404: F, t1123: F, t689: F, t1263: F, t159: F, t635: F, t2251: F, t128: F, t2304: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3351 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk935::<F>(t2257);
        let t3356 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk936::<F>(t1941, t268, t404);
        let (t3357, t3358) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk937::<F>(t3356, t1123, t689);
        let t3360 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk938::<F>(t1263, t159);
        let t3361 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk939::<F>(t635);
        let t3362 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk940::<F>(t3361);
        let t3363 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk941::<F>(t2251, t3362);
        let (t3364, t3365) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk942::<F>(t3360, t3363, t128);
        let t3367 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk943::<F>(t2304);
        let t3368 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk944::<F>(t2251, t3367);
    (t3351, t3356, t3357, t3358, t3360, t3361, t3362, t3363, t3364, t3365, t3367, t3368)
}
