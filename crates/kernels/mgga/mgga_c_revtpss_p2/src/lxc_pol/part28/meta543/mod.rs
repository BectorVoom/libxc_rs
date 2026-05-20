//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta543<F: Float>(t1389: F, t268: F, t10115: F, t555: F, t4146: F, t1398: F, t21990: F, t13790: F, t4056: F, t1882: F, t3923: F, t4003: F) -> (F, F, F, F, F, F, F) {
        let (t46808, t47567, t47672, t48020, t48025, t48073, t48074) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1992::<F>(t1389, t268, t10115, t555, t4146, t1398, t21990, t13790, t4056, t1882, t3923, t4003);
    (t46808, t47567, t47672, t48020, t48025, t48073, t48074)
}
