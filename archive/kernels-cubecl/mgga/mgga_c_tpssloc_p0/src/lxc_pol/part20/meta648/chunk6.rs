//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2385/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2385<F: Float>(t41656: F, t41658: F, t41660: F, t41662: F, t41675: F, t41678: F, t41680: F, t41682: F, t41684: F, t41713: F, t41904: F, t47744: F, t47748: F, t47761: F, t47765: F, t47769: F, t47777: F, t47781: F, t47785: F, t47787: F) -> F {
    let t48980 = F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t47744 + F::cast_from(8.0_f64) * t47748 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t41656 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t41658 + F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t41660 + t41662 / F::cast_from(9.0_f64) + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t41675 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t41678 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t41682 + F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t41684 + F::cast_from(2.0_f64) * t47761 + F::cast_from(2.0_f64) * t47765 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t47769 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t41680 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t41713 + F::cast_from(4.0_f64) * t47777 + t41904 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t47781 - F::cast_from(6.0_f64) * t47785 + F::cast_from(28.0_f64) / F::cast_from(81.0_f64) * t47787;
    t48980
}
