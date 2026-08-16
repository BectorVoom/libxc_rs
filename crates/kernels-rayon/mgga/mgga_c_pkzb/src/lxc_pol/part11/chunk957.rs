//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 957/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk957(t6826: f64, t7012: f64, t10487: f64, t10488: f64, t10489: f64, t10490: f64, t10491: f64, t10492: f64, t4867: f64, t4870: f64, t4876: f64, t4879: f64, t4881: f64, t4884: f64, t5077: f64) -> (f64, f64, f64) {
    let t10493 = 3.0_f64 * t6826;
    let t10494 = 24.0_f64 * t7012;
    let t10495 = t4867 + t4870 - t4876 - t4879 - t10487 + t10488 - t4881 - t10489 - t4884 + t10490 + t10491 + t10492 + t10493 + t5077 - t10494;
    (t10493, t10494, t10495)
}
