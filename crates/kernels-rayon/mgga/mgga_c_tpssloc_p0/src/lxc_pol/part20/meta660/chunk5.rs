//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2469/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2469(t14527: f64, t225: f64, t14534: f64, t10165: f64, t10166: f64, t10167: f64, t10170: f64, t1052: f64, t1066: f64, t13743: f64, t14549: f64, t14555: f64, t14659: f64, t1634: f64, t1635: f64, t3026: f64, t3169: f64, t3175: f64, t3207: f64, t381: f64, t388: f64, t43599: f64, t43604: f64, t4660: f64, t4665: f64, t4693: f64, t48427: f64) -> f64 {
    let t50690 = t14527 * t225;
    let t50703 = t14534 * t225;
    let t50712 = -18.0_f64 * t10165 * t1052 * t3175 * t4693 + 24.0_f64 * t10166 * t1052 * t1634 * t43604 + t381 * t388 * t48427 - 6.0_f64 * t10167 * t4660 + 6.0_f64 * t10170 * t4665 - 3.0_f64 * t1066 * t50690 - 3.0_f64 * t1066 * t50703 + 12.0_f64 * t13743 * t3026 + 12.0_f64 * t13743 * t3169 + 6.0_f64 * t14549 * t3169 - 3.0_f64 * t14555 * t3207 - 3.0_f64 * t14659 * t3026 - 3.0_f64 * t1635 * t43599;
    t50712
}
