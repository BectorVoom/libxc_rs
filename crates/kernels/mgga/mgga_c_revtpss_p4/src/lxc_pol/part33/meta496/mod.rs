//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1801;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1802;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta496<F: Float>(t1401: F, t26024: F, t241: F, t7262: F, t820: F, t3920: F, t7246: F, t2023: F, t2453: F, t3908: F, t72: F, t7307: F, t686: F, t7284: F, t1426: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26025, t26028) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1801::<F>(t1401, t26024, t241, t7262, t820);
        let (t26040, t26041, t26043, t26049, t26050, t26051, t26053, t26054) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1802::<F>(t3920, t7246, t2023, t2453, t3908, t72, t7307, t686, t7284, t1426, t786);
    (t26025, t26028, t26040, t26041, t26043, t26049, t26050, t26051, t26053, t26054)
}
