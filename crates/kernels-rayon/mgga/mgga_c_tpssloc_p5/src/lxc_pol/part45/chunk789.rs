//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 789/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk789(t23384: f64, t6692: f64, t1049: f64, t6688: f64, t6691: f64, t1054: f64, t1065: f64, t1921: f64, t986: f64, t2978: f64, t344: f64, t381: f64) -> (f64, f64, f64, f64, f64) {
    let t23579 = t23384 * t6692;
    let t23581 = t6688 * t1049;
    let t23582 = t23581 * t6691;
    let t23587 = t1054 * t1065;
    let t23588 = t1921 * t23587;
    let t23589 = t986 * t23588;
    let t23592 = t2978 * t344;
    let t23593 = t23592 * t381;
    (t23579, t23582, t23589, t23592, t23593)
}
