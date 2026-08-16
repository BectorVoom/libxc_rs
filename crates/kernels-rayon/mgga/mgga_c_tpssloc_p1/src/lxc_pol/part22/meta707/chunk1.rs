//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2298/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2298(t12725: f64, t19451: f64, t19456: f64, t20100: f64, t20109: f64, t20136: f64, t20717: f64, t2314: f64, t4028: f64, t4034: f64, t4072: f64, t4077: f64, t5107: f64, t5460: f64, t5493: f64, t5494: f64, t6287: f64, t652: f64, t67001: f64, t672: f64, t7458: f64) -> f64 {
    let t67030 = -6.0_f64 * t4072 * t6287 * t652 - 6.0_f64 * t5107 * t5493 * t652 - 12.0_f64 * t12725 * t5460 - 6.0_f64 * t12725 * t5494 - 6.0_f64 * t19451 * t4077 - 12.0_f64 * t19456 * t5460 - 6.0_f64 * t20100 * t4028 - 6.0_f64 * t20100 * t7458 - 12.0_f64 * t20109 * t4028 - 12.0_f64 * t20136 * t7458 - 6.0_f64 * t20717 * t2314 - 6.0_f64 * t20717 * t4034 - 2.0_f64 * t67001 * t672;
    t67030
}
