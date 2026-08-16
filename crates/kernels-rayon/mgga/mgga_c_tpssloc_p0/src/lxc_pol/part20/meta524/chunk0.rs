//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2058/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2058(t12283: f64, t12413: f64, t12267: f64, t3802: f64, t12279: f64, t16398: f64, t12409: f64, t12167: f64, t3792: f64, t1314: f64, t9569: f64, t1329: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39973 = t12283 * t12413;
    let t39975 = t12267 * t3802;
    let t39983 = t16398 * t12279;
    let t39989 = t12283 * t12409;
    let t40000 = t3792 * t12167;
    let t40005 = t9569 * t1314;
    let t40006 = t40005 * t1329;
    (t39973, t39975, t39983, t39989, t40000, t40005, t40006)
}
