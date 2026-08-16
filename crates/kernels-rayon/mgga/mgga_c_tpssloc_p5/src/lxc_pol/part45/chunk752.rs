//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 752/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk752(t23121: f64, t281: f64, t22690: f64, t776: f64, t841: f64, t2617: f64, t6620: f64, t849: f64, t2703: f64, t6621: f64, t6619: f64, t835: f64) -> (f64, f64, f64, f64, f64) {
    let t23122 = t23121 * t281;
    let t23124 = t22690 * t841 * t776;
    let t23125 = t23122 * t23124;
    let t23127 = t2617 * t6620;
    let t23128 = t23127 * t849;
    let t23130 = t6621 * t2703;
    let t23132 = t6619 * t835;
    (t23122, t23125, t23128, t23130, t23132)
}
