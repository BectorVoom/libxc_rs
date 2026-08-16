//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2812/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2812(t4300: f64, t10049: f64, t10110: f64, t13053: f64, t13059: f64, t13461: f64, t1528: f64, t17050: f64, t17064: f64, t17070: f64, t17090: f64, t2597: f64, t2718: f64, t2719: f64, t2743: f64, t4147: f64, t4268: f64, t4273: f64, t4301: f64, t47609: f64, t47618: f64, t5657: f64, t5658: f64, t58143: f64, t58194: f64, t58224: f64, t58261: f64, t58304: f64, t58337: f64, t59351: f64, t59379: f64, t59412: f64, t855: f64, t858: f64, t866: f64, t9590: f64) -> f64 {
    let t59421 = t4300 * t4300;
    let t59434 = -2.0_f64 * t58143 * t866 - 2.0_f64 * t47618 * t1528 - 2.0_f64 * t4268 * t13461 + 8.0_f64 * t13053 * t4273 - 12.0_f64 * t2597 * t17064 - 6.0_f64 * t855 * t10110 * t5657 * t2719 - t10049 * t5658 - t855 * t858 * (t58194 + t58224 + t58261 + t58304 + t58337 + t59351 + t59379 + t59412) - t9590 * t5658 - 4.0_f64 * t13053 * t4301 + 4.0_f64 * t855 * t2718 * t59421 - 2.0_f64 * t2597 * t17050 + 4.0_f64 * t4147 * t13059 + 8.0_f64 * t2597 * t17070 - 4.0_f64 * t47609 * t1528 - t17090 * t2743;
    t59434
}
