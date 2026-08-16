//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2875/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2875<F: Float>(t41658: F, t41675: F, t41684: F, t59655: F, t59657: F, t59661: F, t59663: F, t59665: F, t59670: F, t59674: F, t59678: F, t59680: F, t59684: F) -> F {
    let t60120 = -F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t41658 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t41675 + F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t41684 - F::cast_from(8.0_f64) * t59655 - F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t59657 + F::cast_from(8.0_f64) * t59661 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t59663 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t59665 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t59670 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t59674 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t59678 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t59680 - t59684 / F::cast_from(3.0_f64);
    t60120
}
