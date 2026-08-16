//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2817/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2817(t1484: f64, t2745: f64, t17109: f64, t2752: f64, t13471: f64, t16662: f64, t17116: f64, t1877: f64, t2522: f64, t262: f64, t41254: f64, t41258: f64, t41262: f64, t4307: f64, t4314: f64, t58983: f64, t58985: f64, t58986: f64, t58987: f64, t776: f64, t868: f64) -> f64 {
    let t59580 = t1484 * t2745;
    let t59584 = t17109 * t2752;
    let t59591 = 12.0_f64 * t16662 * t262 * t4314 * t776 - 2.0_f64 * t13471 * t1877 * t4307 - t17116 * t1877 * t2745 - 2.0_f64 * t1877 * t59584 * t868 - 6.0_f64 * t2522 * t4307 * t59580 + t41254 - t41258 - t41262 - t58983 + t58985 - t58986 - t58987;
    t59591
}
