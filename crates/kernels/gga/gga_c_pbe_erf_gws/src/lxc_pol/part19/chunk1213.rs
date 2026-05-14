//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1213/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1213<F: Float>(t4083: F, t9955: F, t12198: F, t1105: F, t353: F, t4228: F, t4386: F, t54952: F, t55796: F, t55807: F, t55809: F, t57488: F, t57495: F, t57497: F, t57500: F, t57506: F, t57509: F, t57514: F, t57516: F, t57542: F, t6793: F, t8793: F) -> (F,) {
    let t58821 = t9955 * t4083;
    let t58823 = t12198 * t4083;
    let t58835 = t4386 * t353 * t4228 * t1105;
    let t58839 = 7.0 / 36.0 * t57488 + t57495 / 384.0 + 7.0 / 288.0 * t58821 + 7.0 / 288.0 * t58823 - t57497 / 48.0 - t57500 / 96.0 - t57506 / 24.0 - t57509 / 48.0 + t57514 / 48.0 + 7.0 / 2304.0 * t57516 + t8793 * t54952 / 24.0 + t6793 * t58835 / 24.0 + t55796 - t55807 - t55809 + 7.0 / 72.0 * t57542;
    (t58839,)
}
