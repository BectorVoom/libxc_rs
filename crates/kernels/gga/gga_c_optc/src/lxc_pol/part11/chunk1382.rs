//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1382/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1382<F: Float>(t389: F, t58614: F, t58626: F, t11894: F, t15064: F, t4281: F, t4289: F, t4297: F, t43649: F, t5229: F, t53390: F, t53432: F, t53443: F, t53445: F, t58322: F, t58581: F, t58585: F, t58591: F, t58596: F) -> (F, F) {
    let t58629 = F::cast_from(0.62182e-1_f64) * (t58614 + t58626) * t389;
    let t58633 = F::cast_from(40000.0_f64) / F::cast_from(81.0_f64) * t15064 * t58581 + F::cast_from(160000.0_f64) / F::cast_from(243.0_f64) * t15064 * t58585 + t58591 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43649 + F::cast_from(200.0_f64) / F::cast_from(27.0_f64) * t53390 * t5229 + F::cast_from(400.0_f64) / F::cast_from(27.0_f64) * t4297 * t58596 - F::cast_from(8.0_f64) * t4281 * t4289 * t11894 * t58322 - t58629 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t53432 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t53443 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t53445;
    (t58629, t58633)
}
