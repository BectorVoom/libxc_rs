//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta455 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1654;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1655;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta455<F: Float>(t159: F, t2698: F, t218: F, t816: F, t228: F, t7021: F, t802: F, t7043: F, t826: F, t2736: F, t2453: F, t7057: F, t1954: F, t9645: F) -> (F, F, F, F, F, F, F, F) {
        let (t25273, t25275, t25277, t25279, t25282, t25283, t25299) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1654::<F>(t159, t2698, t218, t816, t228, t7021, t802, t7043, t826, t2736, t2453, t7057);
        let t25304 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1655::<F>(t1954, t9645);
    (t25273, t25275, t25277, t25279, t25282, t25283, t25299, t25304)
}
