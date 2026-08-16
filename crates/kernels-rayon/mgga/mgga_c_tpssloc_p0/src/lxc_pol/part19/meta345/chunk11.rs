//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1245/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1245(t40672: f64, t40705: f64, t40724: f64, t40756: f64, t40791: f64, t40819: f64, t41591: f64, t41603: f64, t10647: f64, t892: f64, t914: f64, t10650: f64, t2837: f64) -> (f64, f64, f64) {
    let t41606 = t40672 + t40705 + t40724 + t40756 + t40791 + t40819 + t41591 + t41603;
    let t41618 = t10647 * t892;
    let t41620 = 4.0_f64 * t41618 * t914;
    let t41622 = 6.0_f64 * t10650 * t2837;
    (t41606, t41620, t41622)
}
