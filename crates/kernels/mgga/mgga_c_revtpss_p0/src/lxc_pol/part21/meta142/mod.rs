//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta142 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk914;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk915;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk916;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk917;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk918;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk919;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk920;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta142<F: Float>(t2251: F, t3362: F, t3360: F, t128: F, t2304: F, t1120: F, t1121: F, t2258: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t3363 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk914::<F>(t2251, t3362);
        let (t3364, t3365) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk915::<F>(t3360, t3363, t128);
        let t3367 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk916::<F>(t2304);
        let t3368 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk917::<F>(t2251, t3367);
        let (t3369, t3370) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk918::<F>(t1120, t3368, t128);
        let t3372 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk919::<F>(t1121, t2258);
        let (t3373, t3374) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk920::<F>(t1120, t3372, t128);
    (t3363, t3364, t3365, t3367, t3368, t3369, t3370, t3372, t3373, t3374)
}
