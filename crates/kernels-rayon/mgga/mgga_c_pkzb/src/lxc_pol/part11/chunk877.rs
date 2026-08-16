//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 877/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk877(t1979: f64, t3604: f64, t721: f64, t2848: f64, t2852: f64, t5522: f64, t5758: f64, t7357: f64, t7508: f64, t9148: f64, t9163: f64, t261: f64) -> (f64, f64, f64, f64, f64) {
    let t9451 = t3604 * t1979;
    let t9452 = t9451 * t721;
    let t9455 = t2852 * t2848;
    let t9462 = -t5758 + 0.12361111111111111111e-1_f64 * t5522 + 0.24722222222222222223e-1_f64 * t7357 - t7508 - 0.92708333333333333333e-2_f64 * t9148 + 0.278125e-1_f64 * t9163;
    let t9463 = t9462 * t261;
    (t9451, t9452, t9455, t9462, t9463)
}
