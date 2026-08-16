//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1362/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1362(t265: f64, t394: f64, t100497: f64, t105863: f64, t105890: f64, t105934: f64, t105971: f64, t106430: f64, t106460: f64, t106492: f64, t106526: f64, t106606: f64, t1070: f64, t1637: f64, t193: f64, t21376: f64, t21697: f64, t23742: f64, t25840: f64, t336: f64, t4700: f64, t5946: f64, t5950: f64, t6822: f64, t83479: f64, t89702: f64) -> f64 {
    let t395 = t265 < t394;
    let t106607 = piecewise3(t395, t193 * t336 * (t105863 + t105890 + t105934 + t105971 + t106430 + t106460 + t106492 + t106526) * t1070 - 3.0_f64 * t4700 * t100497 * t1637 + 6.0_f64 * t4700 * t89702 * t5950 - 3.0_f64 * t4700 * t25840 * t5946 - 6.0_f64 * t4700 * t83479 * t21376 + 6.0_f64 * t4700 * t23742 * t1637 * t5946 - t4700 * t6822 * t21697, t106606);
    t106607
}
