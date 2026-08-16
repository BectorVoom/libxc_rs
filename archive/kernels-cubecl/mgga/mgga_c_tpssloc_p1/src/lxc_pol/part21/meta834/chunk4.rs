//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2957/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2957<F: Float>(t18057: F, t225: F, t10165: F, t1052: F, t1065: F, t1066: F, t13742: F, t14529: F, t14545: F, t1635: F, t17575: F, t18071: F, t18074: F, t18165: F, t25757: F, t3026: F, t3169: F, t3174: F, t3175: F, t3176: F, t3207: F, t381: F, t388: F, t4694: F, t50622: F, t50628: F, t50690: F, t5943: F, t61058: F, t61061: F, t61618: F) -> F {
    let t61621 = t18057 * t225;
    let t61643 = -F::cast_from(6.0_f64) * t10165 * t1052 * t3175 * t5943 + F::cast_from(4.0_f64) * t1052 * t1065 * t18165 * t3174 - F::cast_from(24.0_f64) * t13742 * t25757 * t50628 + t381 * t388 * t61618 - F::cast_from(4.0_f64) * t1066 * t61058 - F::cast_from(2.0_f64) * t1066 * t61061 - F::cast_from(2.0_f64) * t1066 * t61621 - F::cast_from(4.0_f64) * t14529 * t4694 - F::cast_from(4.0_f64) * t14545 * t4694 - F::cast_from(4.0_f64) * t1635 * t50622 - F::cast_from(2.0_f64) * t1635 * t50690 - t17575 * t3207 - F::cast_from(12.0_f64) * t18071 * t3026 - F::cast_from(12.0_f64) * t18071 * t3169 + F::cast_from(2.0_f64) * t18074 * t3176 - t18074 * t3207;
    t61643
}
