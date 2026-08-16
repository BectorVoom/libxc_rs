//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1081;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta257<F: Float>(t30: F, t890: F, t33: F, t775: F, t1315: F, t196: F, t197: F, t1353: F, t1450: F, t533: F, t7021: F, t816: F, t1941: F, t540: F) -> (F, F, F, F, F, F, F, F) {
        let (t7092, t7200, t7207, t7234, t7235) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1081::<F>(t30, t890, t33, t775, t1315, t196, t197);
        let (t7238, t7250, t7252) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1082::<F>(t1353, t1450, t533, t7021, t816, t1941, t540);
    (t7092, t7200, t7207, t7234, t7235, t7238, t7250, t7252)
}
