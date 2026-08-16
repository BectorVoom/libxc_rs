//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1556/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1556(t15903: f64, t15929: f64, t15981: f64, t16501: f64, t113: f64, t1266: f64, t1271: f64, t12724: f64, t12728: f64, t12835: f64, t12841: f64, t1393: f64, t15857: f64, t1774: f64, t1778: f64, t2312: f64, t2314: f64, t2320: f64, t3929: f64, t4026: f64, t4037: f64, t4077: f64, t510: f64, t5107: f64, t5118: f64, t513: f64, t5361: f64, t650: f64, t652: f64) -> (f64, f64) {
    let t16503 = t15903 + t15929 + t15981 + t16501;
    let t16505 = -t113 * t15857 - 2.0_f64 * t1266 * t4026 + 2.0_f64 * t1271 * t5361 - t12724 * t510 - 2.0_f64 * t12728 * t510 - 2.0_f64 * t12835 * t652 - 2.0_f64 * t12841 * t652 + 2.0_f64 * t1393 * t5118 + t16503 * t513 - t1774 * t2312 - 2.0_f64 * t1774 * t2320 + t1778 * t3929 - 4.0_f64 * t2314 * t4037 - 4.0_f64 * t2314 * t4077 - 2.0_f64 * t5107 * t650;
    (t16503, t16505)
}
