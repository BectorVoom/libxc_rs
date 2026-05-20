//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1289;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1290;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta352<F: Float>(t14485: F, t2465: F, t1558: F, t836: F, t231: F, t2797: F, t2782: F, t860: F, t2783: F, t251: F, t4423: F, t10073: F, t4496: F, t10542: F, t4500: F, t4424: F, t72: F, t686: F, t2798: F, t136: F, t1559: F, t2457: F, t10535: F, t10069: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14486, t14494, t14495, t14498, t14506, t14511, t14512) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1289::<F>(t14485, t2465, t1558, t836, t231, t2797, t2782, t860, t2783, t251, t4423, t10073, t4496);
        let (t14518, t14522, t14525, t14533) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1290::<F>(t10542, t4500, t4424, t72, t686, t2798, t136, t1559, t2457, t10535, t10069, t4496);
    (t14486, t14494, t14495, t14498, t14506, t14511, t14512, t14518, t14522, t14525, t14533)
}
