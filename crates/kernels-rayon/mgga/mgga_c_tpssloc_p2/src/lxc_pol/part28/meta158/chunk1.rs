//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 801/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk801(t3481: f64, t491: f64, t1190: f64, t1235: f64, t1191: f64, t225: f64, t1202: f64, t1226: f64) -> (f64, f64, f64, f64) {
    let t3482 = t3481 * t491;
    let t3484 = t1190 * t1235;
    let t3487 = t1191 * t225;
    let t3490 = t1202 * t1226;
    (t3482, t3484, t3487, t3490)
}
