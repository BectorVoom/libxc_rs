//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1015/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1015<F: Float>(t3740: F, t9507: F, t1145: F, t1161: F, t2821: F, t2834: F, t2838: F, t2922: F, t3688: F, t3720: F, t3724: F, t3739: F, t3757: F, t7637: F, t7643: F, t7734: F, t7775: F, t7800: F, t9453: F, t9458: F, t9468: F, t9475: F, t9479: F, t9482: F, t9485: F, t9490: F, t9493: F, t9504: F) -> (F, F) {
    let t9508 = t3740 * t9507;
    let t9511 = F::cast_from(800.0_f64) / F::cast_from(9.0_f64) * t3720 * t9458 + F::cast_from(800.0_f64) / F::cast_from(9.0_f64) * t3724 * t9458 + F::cast_from(60.0_f64) * t7775 * t1145 * t9468 + F::cast_from(126.0_f64) * t7734 * t9453 - F::cast_from(18.0_f64) * t2922 * t9475 - F::cast_from(56.0_f64) / F::cast_from(3.0_f64) * t7637 * t9479 + F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t2821 * t9482 - F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t2838 * t9485 + F::cast_from(400.0_f64) / F::cast_from(9.0_f64) * t3688 * t9490 - F::cast_from(40.0_f64) / F::cast_from(3.0_f64) * t7800 * t1161 * t9493 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t7643 * t9479 + F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t2834 * t9482 - F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t3757 * t9504 + F::cast_from(352.0_f64) / F::cast_from(243.0_f64) * t3739 * t9508;
    (t9508, t9511)
}
