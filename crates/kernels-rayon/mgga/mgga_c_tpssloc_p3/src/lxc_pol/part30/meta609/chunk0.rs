//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2002/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2002(t23012: f64, t6573: f64, t1883: f64, t82045: f64, t6568: f64, t23205: f64, t82038: f64, t1914: f64, t40772: f64, t1054: f64, t2775: f64, t23326: f64, t6712: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t82211 = t23012 * t6573;
    let t82218 = t82045 * t1883;
    let t82219 = 0.27720185200590482541e0_f64 * t82218;
    let t82259 = t23012 * t6568;
    let t82294 = t82038 * t23205;
    let t82312 = t1914 * t40772;
    let t82342 = t1054 * t2775;
    let t82402 = t6712 * t23326;
    (t82211, t82219, t82259, t82294, t82312, t82342, t82402)
}
