//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2204/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2204(t13977: f64, t13982: f64, t13987: f64, t14189: f64, t23437: f64, t23537: f64, t4596: f64, t4600: f64, t4652: f64, t6765: f64, t82859: f64, t82861: f64, t82863: f64, t82871: f64, t82875: f64, t82877: f64, t83043: f64, t83054: f64, t83061: f64) -> f64 {
    let t88275 = -t83061 * t4600 / 768.0_f64 + t82859 / 1152.0_f64 - t82861 / 2304.0_f64 - t82863 / 324.0_f64 - t23437 * t4652 / 144.0_f64 + 5.0_f64 / 10368.0_f64 * t82871 - t82875 / 5184.0_f64 - t82877 / 1728.0_f64 + 5.0_f64 / 2592.0_f64 * t6765 * t14189 + t83043 * t4596 / 384.0_f64 + t23537 * t13977 / 384.0_f64 + t23537 * t13982 / 768.0_f64 + t83054 * t13987 / 256.0_f64;
    t88275
}
