//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1137/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1137(t32253: f64, t59: f64, t154: f64, t541: f64, t12289: f64, t1336: f64, t835: f64, t1314: f64, t9569: f64, t2559: f64, t3732: f64, t12214: f64, t782: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39933 = t59 * t32253;
    let t39934 = t39933 * t154;
    let t39936 = 455.0_f64 / 243.0_f64 * t39934 * t541;
    let t39944 = t1336 * t12289 * t835;
    let t40005 = t9569 * t1314;
    let t40018 = t2559 * t3732;
    let t40021 = t782 * t12214;
    (t39933, t39934, t39936, t39944, t40005, t40018, t40021)
}
