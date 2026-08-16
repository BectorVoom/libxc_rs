//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2323/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2323(t15418: f64, t2121: f64, t4724: f64, t24720: f64, t27710: f64, t24722: f64, t11588: f64, t4729: f64, t14749: f64, t14753: f64, t15455: f64, t15764: f64, t2140: f64, t3448: f64, t488: f64, t7345: f64, t86341: f64, t86343: f64, t86348: f64, t86350: f64) -> f64 {
    let t95587 = t2121 * t15418 * t4724 / 324.0_f64;
    let t95588 = t27710 * t24720;
    let t95590 = 0.16149102437656156342e-2_f64 * t95588 * t24722;
    let t95593 = t2121 * t11588 * t4729 / 216.0_f64;
    let t95603 = -5.0_f64 / 2592.0_f64 * t7345 * t15455 - t86341 / 864.0_f64 - t86343 / 432.0_f64 + t86348 / 5184.0_f64 - t86350 / 3456.0_f64 + t95587 - t95590 - t95593 - t2121 * t3448 * t14749 / 72.0_f64 - t2121 * t3448 * t14753 / 144.0_f64 + t15764 * t2140 * t488 / 1536.0_f64;
    t95603
}
