//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 661/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk661(t10488: f64, t1801: f64, t1800: f64, t5054: f64, t140: f64, t3737: f64, t4594: f64, t5056: f64, t5049: f64, t5074: f64, t139: f64, t172: f64, t79: f64) -> (f64, f64, f64, f64) {
    let t10489 = t1801 * t10488;
    let t10490 = t1800 * t10489;
    let t10491 = t5054 * t10490;
    let t10494 = t140 * t3737 * t4594;
    let t10495 = t10494 * t5056;
    let t10497 = t5074 * t5049;
    let t10500 = t139 * t172 * t79;
    (t10491, t10495, t10497, t10500)
}
