//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1009/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1009(t1323: f64, t1834: f64, t1811: f64, t3726: f64, t1307: f64, t1810: f64, t210: f64, t119: f64, t5187: f64, t225: f64, t5210: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5217 = t1323 * t1834;
    let t5220 = t3726 * t1811;
    let t5223 = t210 * t1810 * t1307;
    let t5226 = t119 * t5187;
    let t5227 = t210 * t5226;
    let t5230 = t5210 * t225;
    (t5217, t5220, t5223, t5226, t5227, t5230)
}
