//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1193/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1193<F: Float>(t2250: F, t2517: F, t707: F, t751: F, t9449: F, t10121: F, t10126: F, t10134: F, t10143: F, t13487: F, t1877: F, t2522: F, t2553: F, t2745: F, t2749: F, t2752: F, t39373: F, t39397: F, t40674: F, t40677: F, t40679: F, t40681: F, t40683: F, t40685: F, t868: F) -> (F, F, F) {
    let t40687 = t707 * t2517 * t2250;
    let t40688 = F::cast_from(24.0_f64) * t40687;
    let t40689 = t9449 * t751;
    let t40690 = F::cast_from(4.0_f64) * t40689;
    let t40705 = -F::cast_from(4.0_f64) * t10121 * t1877 * t2752 * t868 + F::cast_from(12.0_f64) * t10143 * t1877 * t2745 * t2749 + F::cast_from(18.0_f64) * t10126 * t2522 * t2553 - F::cast_from(36.0_f64) * t10134 * t13487 * t2522 + t39373 - t39397 + t40674 + t40677 - t40679 + t40681 + t40683 - t40685 + t40688 + t40690;
    (t40688, t40690, t40705)
}
