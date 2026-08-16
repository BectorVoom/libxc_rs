//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 624/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk624(t475: f64, t68: f64, t1245: f64, t7375: f64, t1235: f64, t2147: f64, t462: f64, t1215: f64, t2144: f64, t1246: f64, t493: f64, t7348: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7376 = t68 * t475;
    let t7377 = t1245 * t7376;
    let t7378 = t7375 * t7377;
    let t7381 = t2147 * t1235;
    let t7382 = t462 * t7381;
    let t7386 = t2144 * t1215;
    let t7387 = t7386 * t1246;
    let t7389 = t493 * t7348;
    (t7376, t7377, t7378, t7381, t7382, t7387, t7389)
}
