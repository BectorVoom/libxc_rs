//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2705/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2705<F: Float>(t12698: F, t1420: F, t16558: F, t19401: F, t20217: F, t20234: F, t20235: F, t20238: F, t20241: F, t20246: F, t2274: F, t39: F, t39168: F, t39210: F, t3990: F, t3994: F, t43: F, t51: F, t5398: F, t5416: F, t55: F, t607: F, t615: F, t621: F, t67060: F) -> F {
    let t75494 = F::cast_from(5.0_f64) / F::cast_from(162.0_f64) * t51 * t39168 * t20234 * t607 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t12698 * t5398 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t3990 * t16558 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t51 * t2274 * t20217 * t607 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t615 * t20238 + F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t615 * t20235 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t615 * t20241 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t39 * t43 * t67060 + F::cast_from(3080.0_f64) / F::cast_from(81.0_f64) * t20246 * t621 - F::cast_from(220.0_f64) / F::cast_from(9.0_f64) * t5416 * t3994 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t1420 * t19401 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t55 * t67060 - t39210;
    t75494
}
