//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta223 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk953;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk954;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk955;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta223<F: Float>(t45: F, t57: F, t4397: F, t2375: F, t5819: F, t5825: F, t78: F, t2382: F, t81: F, t162: F, t187: F, t150: F, t190: F, t1522: F, t4311: F, zeta_threshold: F, t4399: F, t766: F, t80: F, t770: F, t83: F, t1544: F, t4546: F, t1558: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5927, t5940, t5941, t5943, t5944, t5945, t5947) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk953::<F>(t45, t57, t4397, t2375, t5819, t5825, t78, t2382, t81, t162, t187, t150, t190, t1522, t4311, zeta_threshold);
        let (t5948, t5962) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk954::<F>(t45, t57, t4399, t5819, t5825, t766, t80, t770, t83, zeta_threshold);
        let t5966 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk955::<F>(t1544);
        let (t5970, t5977) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk956::<F>(t1544, t4546, t1558);
    (t5927, t5940, t5941, t5943, t5944, t5945, t5947, t5948, t5962, t5966, t5970, t5977)
}
