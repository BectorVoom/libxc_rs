//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2046/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2046(t25608: f64, t381: f64, t13797: f64, t1926: f64, t221: f64, t10216: f64, t387: f64, t10277: f64, t1625: f64, t225: f64, t344: f64, t25796: f64, t4547: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88004 = t25608 * t381;
    let t88022 = t1926 * t221 * t13797;
    let t88023 = t387 * t10216;
    let t88035 = t387 * t10277;
    let t88050 = t344 * t1625 * t225;
    let t88058 = t4547 * t25796;
    (t88004, t88022, t88023, t88035, t88050, t88058)
}
