//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta501 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1811;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1812;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1813;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1814;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta501<F: Float>(t3707: F, t7617: F, t2134: F, t3682: F, t1234: F, t7623: F, t1210: F, t8945: F, t487: F, t7642: F, t11239: F, t1276: F, t2148: F, t2142: F, t3596: F, t1269: F, t3140: F, t1243: F, t8939: F, t2149: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t26873, t26877, t26880) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1811::<F>(t3707, t7617, t2134, t3682, t1234, t7623);
        let t26889 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1812::<F>(t1210, t8945);
        let (t26894, t26895) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1813::<F>(t487, t7642, t8945);
        let (t26906, t26907, t26918, t26921) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1814::<F>(t11239, t487, t1276, t2148, t2142, t3596, t1269, t3140, t1243, t8939);
        let t26922 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1815::<F>(t2149, t26921);
    (t26873, t26877, t26880, t26889, t26894, t26895, t26906, t26907, t26918, t26921, t26922)
}
