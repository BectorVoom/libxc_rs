//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3016/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3016(t1003: f64, t11037: f64, t11046: f64, t11059: f64, t13940: f64, t14488: f64, t14608: f64, t14615: f64, t14618: f64, t14648: f64, t1629: f64, t1632: f64, t18088: f64, t18117: f64, t18129: f64, t18150: f64, t3120: f64, t3186: f64, t3188: f64, t3200: f64, t353: f64, t360: f64, t383: f64, t43536: f64, t43558: f64, t4673: f64, t4684: f64, t5928: f64, t5939: f64, t62914: f64, t62984: f64, t6739: f64) -> f64 {
    let t63168 = 2.0_f64 * t1003 * t18129 + t353 * t383 * t62914 + 8.0_f64 * t3186 * t18150 * t4673 - 4.0_f64 * t3200 * t18088 * t4684 + t11046 * t5928 * t6739 * t3120 * t360 + 6.0_f64 * t11059 * t5928 * t43558 + 2.0_f64 * t13940 * t1632 + 4.0_f64 * t14618 * t14648 - 2.0_f64 * t11037 * t18117 + 4.0_f64 * t3186 * t1629 * t3188 * t14488 + 4.0_f64 * t3186 * t62984 * t3188 - 4.0_f64 * t14608 * t14615 - t43536 * t5939;
    t63168
}
