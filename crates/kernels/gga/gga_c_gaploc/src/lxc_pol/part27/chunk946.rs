//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 946/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk946<F: Float>(t12290: F, t12315: F, t12317: F, t12321: F, t10285: F, t10288: F, t10289: F, t10297: F, t10300: F, t11130: F, t11132: F, t11134: F, t11137: F, t11139: F, t11140: F, t12033: F, t12149: F, t12277: F, t331: F, t841: F) -> (F, F) {
    let t12323 = t12290 + t12315 + t12317 + t12321;
    let t12325 = -t12277 * t841 + t12323 * t331 + t10285 + t10288 + t10289 + t10297 - t10300 - t11130 + t11132 + t11134 + t11137 + t11139 - t11140 + t12033 + t12149;
    (t12323, t12325)
}
