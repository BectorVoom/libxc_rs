//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1378/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1378<F: Float>(t5087: F, t11596: F, t11597: F, t14849: F, t15064: F, t17516: F, t176: F, t275: F, t277: F, t28031: F, t4281: F, t4297: F, t43584: F, t498: F, t53327: F, t53332: F, t53367: F, t58322: F, t58328: F, t58334: F, t58338: F, t58528: F, t95: F, sigma2: F) -> F {
    let t58535 = t5087 * t5087;
    let t58541 = F::cast_from(176.0_f64) / F::cast_from(9.0_f64) * t53327 + F::cast_from(16000000.0_f64) / F::cast_from(729.0_f64) * t53332 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4281 * t11596 * t11597 * t58322 + F::cast_from(5600.0_f64) / F::cast_from(729.0_f64) * t4297 * t58328 - F::cast_from(400.0_f64) / F::cast_from(27.0_f64) * t14849 * t17516 - F::cast_from(1600.0_f64) / F::cast_from(81.0_f64) * t4297 * t58334 - F::cast_from(80000.0_f64) / F::cast_from(81.0_f64) * t15064 * t58338 + t176 * t58528 * t275 * sigma2 * t498 / F::cast_from(2.0_f64) - F::cast_from(400.0_f64) / F::cast_from(243.0_f64) * t43584 - F::cast_from(0.15506928860942058298e-1_f64) * t95 * t277 * t58535 * t28031 + F::cast_from(136400.0_f64) / F::cast_from(729.0_f64) * t53367;
    t58541
}
