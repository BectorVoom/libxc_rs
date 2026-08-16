//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2651/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2651(t1799: f64, t5356: f64, t20684: f64, t40611: f64, t1390: f64, t20675: f64, t20531: f64, t588: f64, t592: f64, t172: f64, t20396: f64, t763: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74060 = t1799 * t5356;
    let t74064 = t20684 * t40611;
    let t74068 = t20675 * t1390;
    let t74072 = t588 * t20531;
    let t74073 = 4.0_f64 * t74072;
    let t74074 = t592 * t20531;
    let t74075 = 4.0_f64 * t74074;
    let t74077 = t20396 * t172 * t763;
    (t74060, t74064, t74068, t74073, t74075, t74077)
}
