//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1223;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta311<F: Float>(t262: F, t775: F, t3335: F, t389: F, t1077: F, t225: F, t268: F, t271: F, t7021: F, t2435: F, t907: F) -> (F, F, F, F, F, F) {
        let (t11088, t11108, t11121, t11132) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1223::<F>(t262, t775, t3335, t389, t1077, t225, t268, t271, t7021);
        let (t11133, t11134) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1224::<F>(t11132, t2435, t907);
    (t11088, t11108, t11121, t11132, t11133, t11134)
}
