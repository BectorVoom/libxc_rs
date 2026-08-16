//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1015/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1015(t3740: f64, t9507: f64, t1145: f64, t1161: f64, t2821: f64, t2834: f64, t2838: f64, t2922: f64, t3688: f64, t3720: f64, t3724: f64, t3739: f64, t3757: f64, t7637: f64, t7643: f64, t7734: f64, t7775: f64, t7800: f64, t9453: f64, t9458: f64, t9468: f64, t9475: f64, t9479: f64, t9482: f64, t9485: f64, t9490: f64, t9493: f64, t9504: f64) -> (f64, f64) {
    let t9508 = t3740 * t9507;
    let t9511 = 800.0_f64 / 9.0_f64 * t3720 * t9458 + 800.0_f64 / 9.0_f64 * t3724 * t9458 + 60.0_f64 * t7775 * t1145 * t9468 + 126.0_f64 * t7734 * t9453 - 18.0_f64 * t2922 * t9475 - 56.0_f64 / 3.0_f64 * t7637 * t9479 + 88.0_f64 / 27.0_f64 * t2821 * t9482 - 88.0_f64 / 9.0_f64 * t2838 * t9485 + 400.0_f64 / 9.0_f64 * t3688 * t9490 - 40.0_f64 / 3.0_f64 * t7800 * t1161 * t9493 - 8.0_f64 / 3.0_f64 * t7643 * t9479 + 88.0_f64 / 9.0_f64 * t2834 * t9482 - 64.0_f64 / 27.0_f64 * t3757 * t9504 + 352.0_f64 / 243.0_f64 * t3739 * t9508;
    (t9508, t9511)
}
