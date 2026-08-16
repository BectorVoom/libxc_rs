//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2372/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2372<F: Float>(t41655: F, t41656: F, t41658: F, t41660: F, t41662: F, t41675: F, t41678: F, t41680: F, t41682: F, t41684: F, t41713: F, t47744: F, t47748: F, t47761: F, t47765: F, t47769: F, t47777: F, t47781: F, t47785: F, t47787: F) -> F {
    let t48722 = F::cast_from(0.23744444444444444444e0_f64) * t47744 + F::cast_from(0.4274e0_f64) * t47748 - F::cast_from(0.23744444444444444444e-1_f64) * t41656 - F::cast_from(0.15829629629629629629e-1_f64) * t41658 + F::cast_from(0.65956790123456790122e-2_f64) * t41660 + F::cast_from(0.5936111111111111111e-2_f64) * t41662 + F::cast_from(0.47488888888888888887e-1_f64) * t41675 - F::cast_from(0.23744444444444444444e-1_f64) * t41678 + F::cast_from(0.35616666666666666666e-1_f64) * t41682 + F::cast_from(0.55403703703703703702e-1_f64) * t41684 + F::cast_from(0.10685e0_f64) * t47761 + F::cast_from(0.10685e0_f64) * t47765 + F::cast_from(0.35616666666666666666e-1_f64) * t47769 + F::cast_from(0.11872222222222222222e-1_f64) * t41680 - F::cast_from(0.35616666666666666666e-1_f64) * t41713 + F::cast_from(0.21369999999999999999e0_f64) * t47777 + t41655 - F::cast_from(0.59361111111111111111e-1_f64) * t47781 - F::cast_from(0.32055e0_f64) * t47785 + F::cast_from(0.18467901234567901234e-1_f64) * t47787;
    t48722
}
