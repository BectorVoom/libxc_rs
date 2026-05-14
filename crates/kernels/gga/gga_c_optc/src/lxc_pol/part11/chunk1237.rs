//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1237/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1237<F: Float>(t389: F, t58614: F, t58626: F, t11894: F, t15064: F, t4281: F, t4289: F, t4297: F, t43649: F, t5229: F, t53390: F, t53432: F, t53443: F, t53445: F, t58322: F, t58581: F, t58585: F, t58591: F, t58596: F) -> (F, F) {
    let t58629 = 0.62182e-1 * (t58614 + t58626) * t389;
    let t58633 = 40000.0 / 81.0 * t15064 * t58581 + 160000.0 / 243.0 * t15064 * t58585 + t58591 + 4.0 / 9.0 * t43649 + 200.0 / 27.0 * t53390 * t5229 + 400.0 / 27.0 * t4297 * t58596 - 8.0 * t4281 * t4289 * t11894 * t58322 - t58629 + 32.0 / 9.0 * t53432 + 8.0 / 9.0 * t53443 - 16.0 / 9.0 * t53445;
    (t58629, t58633)
}
