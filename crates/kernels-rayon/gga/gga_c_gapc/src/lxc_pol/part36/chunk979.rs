//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 979/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk979(t291: f64, t3137: f64, t959: f64, t7191: f64, t11834: f64, t1026: f64, t932: f64, t3304: f64, t3285: f64, t3775: f64, t3289: f64, t19: f64, t825: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11836 = t3137 * t291 * t959;
    let t11837 = t11836 * t7191;
    let t11838 = t11834 * t11837;
    let t11840 = t932 * t1026;
    let t11841 = t11840 * t3304;
    let t11843 = t3775 * t3285;
    let t11845 = t3775 * t3289;
    let t11847 = t825 * t19;
    (t11837, t11838, t11840, t11841, t11843, t11845, t11847)
}
