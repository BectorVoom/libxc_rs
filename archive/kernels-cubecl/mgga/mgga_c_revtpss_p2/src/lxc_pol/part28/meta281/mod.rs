//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1254;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1255;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1256;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1257;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta281<F: Float>(t128: F, t121: F, t22: F, t2508: F, t9285: F, t692: F, t9288: F, t124: F, t624: F, t138: F, t9283: F, t9286: F, t9289: F, t9292: F, t701: F, t682: F, t2580: F, t680: F, t130: F, t146: F, t2583: F, t9275: F, t2514: F, t2596: F, t746: F, t1340: F, t2491: F, t2495: F, t744: F, t215: F, t681: F, t268: F, t702: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9296, t9298, t9300, t9302, t9303) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1254::<F>(t128, t121, t22, t2508, t9285, t692, t9288, t124, t624, t138);
        let t9308 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1255::<F>(t9283, t9286, t9289, t9292, t9296, t9298, t9300, t9303, t701, t682);
        let t9316 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1256::<F>(t2580, t680, t130, t146, t2583, t9275);
        let (t9318, t9320, t9323, t9325, t9329) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1257::<F>(t2514, t2596, t746, t1340, t2491, t2495, t744, t215, t681, t268, t702);
    (t9296, t9298, t9300, t9302, t9303, t9308, t9316, t9318, t9320, t9323, t9325, t9329)
}
