//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 845/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk845(t1685: f64, t28341: f64, t4787: f64, t22760: f64, t7509: f64, t22891: f64, t2382: f64, t6802: f64, t8574: f64, t16356: f64, t8577: f64, t2381: f64, t8549: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28343 = t4787 * t28341 * t1685;
    let t28346 = t22760 * t7509;
    let t28352 = 3.0_f64 * t22891 * t2382;
    let t28354 = 3.0_f64 * t6802 * t8574;
    let t28356 = 0.48245472966453314466e2_f64 * t16356 * t8577;
    let t28357 = t8549 * t2381;
    (t28343, t28346, t28352, t28354, t28356, t28357)
}
