//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1254/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1254(t40779: f64, t40786: f64, t40788: f64, t41859: f64, t41867: f64, t41870: f64, t41871: f64, t42491: f64, t42493: f64, t42495: f64, t42497: f64, t42500: f64, t42502: f64, t42505: f64, t42508: f64, t42512: f64, t42516: f64, t42519: f64) -> f64 {
    let t44630 = t42491 + t42493 - 3.0_f64 / 2.0_f64 * t42495 + t42497 / 2.0_f64 + t42500 / 2.0_f64 - 8.0_f64 / 3.0_f64 * t42502 + 4.0_f64 * t42505 - 4.0_f64 / 3.0_f64 * t42508 - 44.0_f64 / 9.0_f64 * t40779 + t41859 + t40786 + 88.0_f64 / 9.0_f64 * t40788 - t41867 - t41870 - t41871 - 3.0_f64 * t42512 + 6.0_f64 * t42516 - 3.0_f64 * t42519;
    t44630
}
