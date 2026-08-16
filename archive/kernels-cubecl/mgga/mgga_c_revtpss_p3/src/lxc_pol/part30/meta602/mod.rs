//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2063;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta602<F: Float>(t13085: F, t7624: F, t13017: F, t7607: F, t12901: F, t26844: F, t13014: F, t12998: F, t26866: F, t3746: F, t12773: F, t26867: F) -> (F, F, F, F, F, F, F) {
        let (t97200, t97204, t97218, t97220, t97222, t97232, t97239) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2063::<F>(t13085, t7624, t13017, t7607, t12901, t26844, t13014, t12998, t26866, t3746, t12773, t26867);
    (t97200, t97204, t97218, t97220, t97222, t97232, t97239)
}
