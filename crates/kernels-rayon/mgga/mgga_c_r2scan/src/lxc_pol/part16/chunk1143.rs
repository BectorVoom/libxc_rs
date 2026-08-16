//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1143/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1143(t37078: f64, t40782: f64, t40798: f64, t40805: f64, t40807: f64, t41858: f64, t41864: f64, t42491: f64, t42493: f64, t42495: f64, t42497: f64, t42500: f64, t42502: f64, t42505: f64, t42508: f64, t42512: f64, t42516: f64, t42519: f64) -> f64 {
    let t42521 = t42491 / 2.0_f64 + t42493 / 2.0_f64 - 3.0_f64 / 4.0_f64 * t42495 + t42497 / 4.0_f64 + t42500 / 4.0_f64 - 4.0_f64 / 3.0_f64 * t42502 + 2.0_f64 * t42505 - 2.0_f64 / 3.0_f64 * t42508 - t41858 + t40782 + 22.0_f64 / 9.0_f64 * t37078 + t41864 - t40798 - t40805 - t40807 - 3.0_f64 / 2.0_f64 * t42512 + 3.0_f64 * t42516 - 3.0_f64 / 2.0_f64 * t42519;
    t42521
}
