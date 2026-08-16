//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1115/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1115(t3030: f64, t344: f64, t1014: f64, t1011: f64, t360: f64, t1949: f64, t2966: f64, t1920: f64, t210: f64, t6795: f64, t6688: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23602 = t344 * t3030;
    let t23603 = t23602 * t1014;
    let t23604 = t1011 * t360;
    let t23617 = t2966 * t1949;
    let t23619 = 0.18277045187202515961e-2_f64 * t1920 * t23617;
    let t23631 = t6795 * t210;
    let t23632 = t974 * t6688;
    (t23602, t23603, t23604, t23619, t23631, t23632)
}
