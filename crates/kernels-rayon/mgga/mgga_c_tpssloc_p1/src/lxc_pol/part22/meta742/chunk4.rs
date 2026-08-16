//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2456/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2456(t69533: f64, t69574: f64, t69665: f64, t69695: f64, t69741: f64, t69791: f64, t69817: f64, t69837: f64, t1049: f64, t1052: f64, t1065: f64, t1625: f64, t1635: f64, t17583: f64, t17588: f64, t17875: f64, t18071: f64, t18166: f64, t21480: f64, t21662: f64, t21663: f64, t3026: f64, t3174: f64, t381: f64, t388: f64, t4557: f64, t4660: f64, t4665: f64, t4694: f64, t61058: f64) -> (f64, f64) {
    let t69840 = t69533 + t69574 + t69665 + t69695 + t69741 + t69791 + t69817 + t69837;
    let t69860 = 2.0_f64 * t1052 * t1065 * t21662 * t3174 + t1049 * t21480 * t388 + 3.0_f64 * t1625 * t17875 * t388 + t381 * t388 * t69840 - 6.0_f64 * t1635 * t61058 + 12.0_f64 * t17583 * t4557 + 12.0_f64 * t17583 * t4660 + 12.0_f64 * t17588 * t4665 - 6.0_f64 * t17588 * t4694 - 18.0_f64 * t18071 * t4557 - 18.0_f64 * t18071 * t4660 - 3.0_f64 * t18166 * t4660 - t21663 * t3026;
    (t69840, t69860)
}
