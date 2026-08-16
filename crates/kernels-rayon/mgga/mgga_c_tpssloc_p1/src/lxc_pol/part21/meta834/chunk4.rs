//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2957/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2957(t18057: f64, t225: f64, t10165: f64, t1052: f64, t1065: f64, t1066: f64, t13742: f64, t14529: f64, t14545: f64, t1635: f64, t17575: f64, t18071: f64, t18074: f64, t18165: f64, t25757: f64, t3026: f64, t3169: f64, t3174: f64, t3175: f64, t3176: f64, t3207: f64, t381: f64, t388: f64, t4694: f64, t50622: f64, t50628: f64, t50690: f64, t5943: f64, t61058: f64, t61061: f64, t61618: f64) -> f64 {
    let t61621 = t18057 * t225;
    let t61643 = -6.0_f64 * t10165 * t1052 * t3175 * t5943 + 4.0_f64 * t1052 * t1065 * t18165 * t3174 - 24.0_f64 * t13742 * t25757 * t50628 + t381 * t388 * t61618 - 4.0_f64 * t1066 * t61058 - 2.0_f64 * t1066 * t61061 - 2.0_f64 * t1066 * t61621 - 4.0_f64 * t14529 * t4694 - 4.0_f64 * t14545 * t4694 - 4.0_f64 * t1635 * t50622 - 2.0_f64 * t1635 * t50690 - t17575 * t3207 - 12.0_f64 * t18071 * t3026 - 12.0_f64 * t18071 * t3169 + 2.0_f64 * t18074 * t3176 - t18074 * t3207;
    t61643
}
