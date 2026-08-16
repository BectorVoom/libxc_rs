//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3016/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3016<F: Float>(t1003: F, t11037: F, t11046: F, t11059: F, t13940: F, t14488: F, t14608: F, t14615: F, t14618: F, t14648: F, t1629: F, t1632: F, t18088: F, t18117: F, t18129: F, t18150: F, t3120: F, t3186: F, t3188: F, t3200: F, t353: F, t360: F, t383: F, t43536: F, t43558: F, t4673: F, t4684: F, t5928: F, t5939: F, t62914: F, t62984: F, t6739: F) -> F {
    let t63168 = F::cast_from(2.0_f64) * t1003 * t18129 + t353 * t383 * t62914 + F::cast_from(8.0_f64) * t3186 * t18150 * t4673 - F::cast_from(4.0_f64) * t3200 * t18088 * t4684 + t11046 * t5928 * t6739 * t3120 * t360 + F::cast_from(6.0_f64) * t11059 * t5928 * t43558 + F::cast_from(2.0_f64) * t13940 * t1632 + F::cast_from(4.0_f64) * t14618 * t14648 - F::cast_from(2.0_f64) * t11037 * t18117 + F::cast_from(4.0_f64) * t3186 * t1629 * t3188 * t14488 + F::cast_from(4.0_f64) * t3186 * t62984 * t3188 - F::cast_from(4.0_f64) * t14608 * t14615 - t43536 * t5939;
    t63168
}
