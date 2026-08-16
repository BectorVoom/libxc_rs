//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1155/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1155(t1131: f64, t1175: f64, t13885: f64, t13886: f64, t14163: f64, t1901: f64, t21362: f64, t21399: f64, t21486: f64, t21504: f64, t2574: f64, t2594: f64, t265: f64, t42362: f64, t446: f64, t4965: f64, t5053: f64, t5073: f64, t5181: f64, t53923: f64, t68662: f64, t724: f64, t729: f64, t81730: f64, t81780: f64, t88735: f64, t89083: f64) -> f64 {
    let t89608 = -8.0_f64 / 9.0_f64 * t81730 + 8.0_f64 / 3.0_f64 * t446 * t2574 * t265 * t1131 * t21399 - 8.0_f64 / 3.0_f64 * t1901 * t14163 * t89083 - 8.0_f64 / 3.0_f64 * t1901 * t53923 * t21504 - 2.0_f64 * t446 * t729 * t5181 * t5053 - 8.0_f64 / 9.0_f64 * t1901 * t42362 * t4965 * t5073 + 16.0_f64 / 27.0_f64 * t68662 - 8.0_f64 * t1901 * t13885 * t13886 * t21486 - 8.0_f64 / 3.0_f64 * t446 * t724 * t1175 * t21362 - 8.0_f64 / 3.0_f64 * t446 * t2594 * t265 * t88735 + 4.0_f64 / 27.0_f64 * t81780;
    t89608
}
