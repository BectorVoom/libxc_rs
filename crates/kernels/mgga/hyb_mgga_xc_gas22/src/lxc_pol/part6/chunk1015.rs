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
    let t9511 = F::new(800.0) / F::new(9.0) * t3720 * t9458 + F::new(800.0) / F::new(9.0) * t3724 * t9458 + F::new(60.0) * t7775 * t1145 * t9468 + F::new(126.0) * t7734 * t9453 - F::new(18.0) * t2922 * t9475 - F::new(56.0) / F::new(3.0) * t7637 * t9479 + F::new(88.0) / F::new(27.0) * t2821 * t9482 - F::new(88.0) / F::new(9.0) * t2838 * t9485 + F::new(400.0) / F::new(9.0) * t3688 * t9490 - F::new(40.0) / F::new(3.0) * t7800 * t1161 * t9493 - F::new(8.0) / F::new(3.0) * t7643 * t9479 + F::new(88.0) / F::new(9.0) * t2834 * t9482 - F::new(64.0) / F::new(27.0) * t3757 * t9504 + F::new(352.0) / F::new(243.0) * t3739 * t9508;
    (t9508, t9511)
}
