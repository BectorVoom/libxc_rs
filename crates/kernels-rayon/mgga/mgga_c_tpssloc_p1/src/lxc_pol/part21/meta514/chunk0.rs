//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2162/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2162(t17635: f64, t4588: f64, t4582: f64, t1023: f64, t5681: f64, t3071: f64, t248: f64, t3101: f64, t5878: f64, t3039: f64, t3051: f64, t5685: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17642 = t4588 * t17635;
    let t17643 = t4582 * t17642;
    let t17648 = t5681 * t1023;
    let t17649 = t3071 * t17648;
    let t17655 = t248 * t3101 * t5878;
    let t17656 = t3039 * t17655;
    let t17659 = t248 * t3051 * t5685;
    (t17642, t17643, t17648, t17649, t17655, t17656, t17659)
}
