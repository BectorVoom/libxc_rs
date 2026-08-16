//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1685;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1686;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta462<F: Float>(t2018: F, t3951: F, t807: F, t1941: F, t550: F, t1389: F, t25240: F, t3964: F, t7262: F, t820: F, t843: F, t1401: F, t241: F) -> (F, F, F, F, F, F, F) {
        let (t26014, t26016, t26017, t26021, t26024) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1685::<F>(t2018, t3951, t807, t1941, t550, t1389, t25240, t3964, t7262, t820, t843);
        let (t26025, t26028) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1686::<F>(t1401, t26024, t241, t7262, t820);
    (t26014, t26016, t26017, t26021, t26024, t26025, t26028)
}
