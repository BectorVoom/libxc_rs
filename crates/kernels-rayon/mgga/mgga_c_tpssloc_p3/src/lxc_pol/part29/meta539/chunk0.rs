//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1930/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1930(t1998: f64, t5318: f64, t214: f64, t1985: f64, t7740: f64, t794: f64, t6897: f64, t1825: f64, t22873: f64, t552: f64, t6604: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26432 = t1998 * t5318;
    let t26433 = t214 * t26432;
    let t26434 = t1985 * t26433;
    let t26436 = t794 * t7740;
    let t26437 = t6897 * t26436;
    let t26442 = t22873 * t1825;
    let t26446 = t6604 * t552;
    (t26432, t26433, t26434, t26436, t26437, t26442, t26446)
}
