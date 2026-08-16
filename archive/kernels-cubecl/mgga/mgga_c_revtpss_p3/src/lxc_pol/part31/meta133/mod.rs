//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta133 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk727;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk728;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk729;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk730;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk731;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk732;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta133<F: Float>(t3303: F, t357: F, t389: F, t1941: F, t268: F, t404: F, t1123: F, t689: F, t1263: F, t159: F, t635: F, t2304: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3318, t3335, t3336) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk727::<F>(t3303, t357, t389);
        let t3356 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk728::<F>(t1941, t268, t404);
        let (t3357, t3358) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk729::<F>(t3356, t1123, t689);
        let t3360 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk730::<F>(t1263, t159);
        let (t3361, t3362) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk731::<F>(t635);
        let t3367 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk732::<F>(t2304);
    (t3318, t3335, t3336, t3356, t3357, t3358, t3360, t3361, t3362, t3367)
}
