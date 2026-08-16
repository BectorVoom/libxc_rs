//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2666/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2666(t12725: f64, t12734: f64, t12823: f64, t12841: f64, t1442: f64, t1459: f64, t15857: f64, t19456: f64, t20109: f64, t20143: f64, t2314: f64, t2320: f64, t3652: f64, t4028: f64, t4034: f64, t4037: f64, t4072: f64, t510: f64, t5107: f64, t5118: f64, t5361: f64, t5457: f64, t5460: f64, t5494: f64, t55946: f64, t55962: f64, t55967: f64, t6287: f64, t652: f64) -> f64 {
    let t56034 = -8.0_f64 * t4072 * t5107 * t652 - 8.0_f64 * t12725 * t4037 - 4.0_f64 * t12734 * t5494 - 4.0_f64 * t12823 * t5460 - 2.0_f64 * t12823 * t5494 - 4.0_f64 * t12841 * t4028 - 2.0_f64 * t1442 * t15857 - 4.0_f64 * t1459 * t55962 - 8.0_f64 * t19456 * t4037 - 8.0_f64 * t20109 * t4034 - 4.0_f64 * t20143 * t2314 - 4.0_f64 * t20143 * t4034 - 2.0_f64 * t2320 * t6287 - 2.0_f64 * t3652 * t5457 - 2.0_f64 * t510 * t55946 - 2.0_f64 * t510 * t55967 + 4.0_f64 * t5118 * t5361;
    t56034
}
