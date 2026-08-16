//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1391;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1392;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1393;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta363<F: Float>(t14495: F, t2797: F, t2782: F, t1558: F, t860: F, t231: F, t2783: F, t251: F, t4423: F, t10073: F, t4496: F, t10542: F, t4500: F, t4424: F, t72: F, t686: F, t2798: F, t136: F, t1559: F, t2457: F, t10535: F, t10069: F, t1568: F, t836: F, t10867: F, t225: F, t213: F, t2777: F, t4518: F, t2439: F, t2470: F, t4499: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14498, t14506, t14511, t14512, t14518) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1391::<F>(t14495, t2797, t2782, t1558, t860, t231, t2783, t251, t4423, t10073, t4496, t10542, t4500);
        let (t14522, t14525, t14533, t14535) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1392::<F>(t4424, t72, t686, t2798, t136, t1559, t2457, t10535, t10069, t4496, t1568, t836);
        let (t14539, t14546, t14558, t14563) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1393::<F>(t14535, t231, t2783, t2782, t10867, t225, t213, t2777, t4518, t2439, t2470, t4499);
    (t14498, t14506, t14511, t14512, t14518, t14522, t14525, t14533, t14539, t14546, t14558, t14563)
}
