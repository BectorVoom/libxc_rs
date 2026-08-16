//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2203/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2203(t14025: f64, t23537: f64, t13970: f64, t23541: f64, t13991: f64, t14107: f64, t14143: f64, t14147: f64, t14180: f64, t14184: f64, t14235: f64, t23419: f64, t23529: f64, t4585: f64, t4590: f64, t6765: f64, t82843: f64, t82851: f64, t83058: f64, t83065: f64) -> f64 {
    let t88249 = t23537 * t14025 / 576.0_f64;
    let t88251 = t23541 * t13970 / 1152.0_f64;
    let t88254 = t23529 * t4585 / 108.0_f64 - 5.0_f64 / 648.0_f64 * t23529 * t4590 + 5.0_f64 / 3456.0_f64 * t6765 * t14180 + 5.0_f64 / 6912.0_f64 * t6765 * t14184 + t83065 * t14107 / 1536.0_f64 - t6765 * t14143 / 576.0_f64 - t6765 * t14147 / 1152.0_f64 + 5.0_f64 / 3456.0_f64 * t23419 * t14235 + t82843 / 3456.0_f64 - t82851 / 3456.0_f64 + t88249 - t88251 - t83058 * t13991 / 256.0_f64;
    t88254
}
