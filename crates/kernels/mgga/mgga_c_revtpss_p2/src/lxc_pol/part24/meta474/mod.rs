//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta474<F: Float>(t2439: F, t6132: F, t6135: F, t6138: F, t2873: F, t6104: F, t11108: F, t6396: F, t11452: F, t6173: F, t2986: F, t6184: F) -> (F, F, F, F, F, F, F) {
        let (t63533, t63538, t63545, t63677, t63907, t63979, t63997) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1457::<F>(t2439, t6132, t6135, t6138, t2873, t6104, t11108, t6396, t11452, t6173, t2986, t6184);
    (t63533, t63538, t63545, t63677, t63907, t63979, t63997)
}
