//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 877/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk877(t8662: f64, t9231: f64, t9239: f64, t131: f64, t7245: f64, t2240: f64, t7254: f64, t8301: f64, t3701: f64, t7216: f64, t2039: f64, t7408: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31857 = t9231 * t8662;
    let t31860 = t9239 * t8662;
    let t31863 = t7245 * t131;
    let t31864 = t2240 * t31863;
    let t31867 = t8301 * t7254;
    let t31868 = t2240 * t31867;
    let t32193 = t3701 * t7216;
    let t32318 = t7408 * t2039;
    (t31857, t31860, t31863, t31864, t31867, t31868, t32193, t32318)
}
