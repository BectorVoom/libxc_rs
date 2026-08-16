//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1193/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1193(t2250: f64, t2517: f64, t707: f64, t751: f64, t9449: f64, t10121: f64, t10126: f64, t10134: f64, t10143: f64, t13487: f64, t1877: f64, t2522: f64, t2553: f64, t2745: f64, t2749: f64, t2752: f64, t39373: f64, t39397: f64, t40674: f64, t40677: f64, t40679: f64, t40681: f64, t40683: f64, t40685: f64, t868: f64) -> (f64, f64, f64) {
    let t40687 = t707 * t2517 * t2250;
    let t40688 = 24.0_f64 * t40687;
    let t40689 = t9449 * t751;
    let t40690 = 4.0_f64 * t40689;
    let t40705 = -4.0_f64 * t10121 * t1877 * t2752 * t868 + 12.0_f64 * t10143 * t1877 * t2745 * t2749 + 18.0_f64 * t10126 * t2522 * t2553 - 36.0_f64 * t10134 * t13487 * t2522 + t39373 - t39397 + t40674 + t40677 - t40679 + t40681 + t40683 - t40685 + t40688 + t40690;
    (t40688, t40690, t40705)
}
