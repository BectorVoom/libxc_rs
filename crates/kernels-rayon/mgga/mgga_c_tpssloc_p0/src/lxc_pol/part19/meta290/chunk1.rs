//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1062/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1062(t1307: f64, t3792: f64, t12368: f64, t3805: f64, t1328: f64, t210: f64, t3719: f64, t12178: f64, t1343: f64, t820: f64, t3788: f64, t835: f64) -> (f64, f64, f64, f64, f64) {
    let t12369 = t3792 * t1307;
    let t12371 = t3805 * t12368 * t12369;
    let t12375 = t210 * t1328 * t3719;
    let t12379 = t1343 * t820 * t12178;
    let t12384 = t3788 * t835;
    (t12369, t12371, t12375, t12379, t12384)
}
