//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1378/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1378(t5087: f64, t11596: f64, t11597: f64, t14849: f64, t15064: f64, t17516: f64, t176: f64, t275: f64, t277: f64, t28031: f64, t4281: f64, t4297: f64, t43584: f64, t498: f64, t53327: f64, t53332: f64, t53367: f64, t58322: f64, t58328: f64, t58334: f64, t58338: f64, t58528: f64, t95: f64, sigma2: f64) -> f64 {
    let t58535 = t5087 * t5087;
    let t58541 = 176.0_f64 / 9.0_f64 * t53327 + 16000000.0_f64 / 729.0_f64 * t53332 + 28.0_f64 / 9.0_f64 * t4281 * t11596 * t11597 * t58322 + 5600.0_f64 / 729.0_f64 * t4297 * t58328 - 400.0_f64 / 27.0_f64 * t14849 * t17516 - 1600.0_f64 / 81.0_f64 * t4297 * t58334 - 80000.0_f64 / 81.0_f64 * t15064 * t58338 + t176 * t58528 * t275 * sigma2 * t498 / 2.0_f64 - 400.0_f64 / 243.0_f64 * t43584 - 0.15506928860942058298e-1_f64 * t95 * t277 * t58535 * t28031 + 136400.0_f64 / 729.0_f64 * t53367;
    t58541
}
