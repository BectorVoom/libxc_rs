//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1253;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1254;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1255;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1256;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta286<F: Float>(t9283: F, t9286: F, t9289: F, t9292: F, t9296: F, t9298: F, t9300: F, t9303: F, t701: F, t682: F, t2580: F, t680: F, t130: F, t146: F, t2583: F, t9275: F, t2514: F, t2596: F, t746: F, t1340: F, t2491: F, t2495: F, t744: F, t215: F, t681: F, t268: F, t702: F, t2564: F, t2567: F, t675: F) -> (F, F, F, F, F, F, F, F) {
        let t9308 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1253::<F>(t9283, t9286, t9289, t9292, t9296, t9298, t9300, t9303, t701, t682);
        let t9316 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1254::<F>(t2580, t680, t130, t146, t2583, t9275);
        let (t9318, t9320, t9323, t9325, t9329) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1255::<F>(t2514, t2596, t746, t1340, t2491, t2495, t744, t215, t681, t268, t702);
        let t9333 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1256::<F>(t2564, t2567, t268, t675);
    (t9308, t9316, t9318, t9320, t9323, t9325, t9329, t9333)
}
