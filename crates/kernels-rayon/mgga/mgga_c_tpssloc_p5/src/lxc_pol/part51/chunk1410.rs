//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1410/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1410(t33476: f64, t776: f64, t114992: f64, t115009: f64, t118387: f64, t1877: f64, t22960: f64, t24339: f64, t25024: f64, t25028: f64, t2522: f64, t25366: f64, t25377: f64, t25385: f64, t26563: f64, t26744: f64, t31430: f64, t31434: f64, t31451: f64, t33486: f64, t7114: f64, t7475: f64, t7545: f64, t8566: f64) -> (f64, f64) {
    let t121818 = t33476 * t776;
    let t121833 = -t1877 * t7114 * t118387 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t8566 * t25028 + 3.0_f64 / 2.0_f64 * t2522 * t8566 * t25385 - t1877 * t24339 * t33486 / 2.0_f64 - t1877 * t26744 * t31451 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t8566 * t25024 - 3.0_f64 * t26563 * t22960 * t121818 + 3.0_f64 / 2.0_f64 * t2522 * t31430 * t7475 - t1877 * t114992 * t7545 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t115009 * t25366 - t1877 * t31434 * t25377 / 2.0_f64;
    (t121818, t121833)
}
