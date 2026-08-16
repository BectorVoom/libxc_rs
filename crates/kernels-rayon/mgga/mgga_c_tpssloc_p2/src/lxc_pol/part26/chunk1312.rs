//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1312/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1312(t225: f64, t23202: f64, t6556: f64, t81632: f64, t23012: f64, t6573: f64, t1883: f64, t82045: f64, t23164: f64, t6555: f64, t82133: f64, t23270: f64, t2379: f64, t25038: f64, t857: f64, t865: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82197 = t23202 * t225;
    let t82209 = t81632 * t6556;
    let t82211 = t23012 * t6573;
    let t82218 = t82045 * t1883;
    let t82219 = 0.27720185200590482541e0_f64 * t82218;
    let t82221 = t23164 * t82133 * t6555;
    let t82228 = t25038 * t23270 * t857 * t2379 * t865;
    (t82197, t82209, t82211, t82219, t82221, t82228)
}
