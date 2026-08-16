//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2812/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2812<F: Float>(t4300: F, t10049: F, t10110: F, t13053: F, t13059: F, t13461: F, t1528: F, t17050: F, t17064: F, t17070: F, t17090: F, t2597: F, t2718: F, t2719: F, t2743: F, t4147: F, t4268: F, t4273: F, t4301: F, t47609: F, t47618: F, t5657: F, t5658: F, t58143: F, t58194: F, t58224: F, t58261: F, t58304: F, t58337: F, t59351: F, t59379: F, t59412: F, t855: F, t858: F, t866: F, t9590: F) -> F {
    let t59421 = t4300 * t4300;
    let t59434 = -F::cast_from(2.0_f64) * t58143 * t866 - F::cast_from(2.0_f64) * t47618 * t1528 - F::cast_from(2.0_f64) * t4268 * t13461 + F::cast_from(8.0_f64) * t13053 * t4273 - F::cast_from(12.0_f64) * t2597 * t17064 - F::cast_from(6.0_f64) * t855 * t10110 * t5657 * t2719 - t10049 * t5658 - t855 * t858 * (t58194 + t58224 + t58261 + t58304 + t58337 + t59351 + t59379 + t59412) - t9590 * t5658 - F::cast_from(4.0_f64) * t13053 * t4301 + F::cast_from(4.0_f64) * t855 * t2718 * t59421 - F::cast_from(2.0_f64) * t2597 * t17050 + F::cast_from(4.0_f64) * t4147 * t13059 + F::cast_from(8.0_f64) * t2597 * t17070 - F::cast_from(4.0_f64) * t47609 * t1528 - t17090 * t2743;
    t59434
}
