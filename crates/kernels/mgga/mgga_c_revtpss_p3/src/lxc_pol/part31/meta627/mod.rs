//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta627 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2080;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2081;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta627<F: Float>(t1078: F, t1982: F, t3140: F, t4930: F, t25604: F, t7825: F, t1678: F, t7150: F, t8521: F, t27418: F, t3057: F, t3046: F, t7810: F, t27543: F, t994: F, t1977: F, t11200: F, t7143: F, t15827: F, t27536: F, t15904: F, t25515: F, t12047: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t99886, t99909, t99915, t99934, t99940) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2080::<F>(t1078, t1982, t3140, t4930, t25604, t7825, t1678, t7150, t8521, t27418, t3057, t3046, t7810);
        let (t99947, t99953, t99969, t99983, t99984, t99985) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2081::<F>(t27543, t994, t1977, t3057, t1078, t11200, t7143, t15827, t27536, t15904, t25515, t12047);
    (t99886, t99909, t99915, t99934, t99940, t99947, t99953, t99969, t99983, t99984, t99985)
}
