//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 738/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk738(t5942: f64, t615: f64, t1757: f64, t1679: f64, t584: f64, t1685: f64, t591: f64, t1684: f64, t2065: f64, t595: f64, t637: f64, t2068: f64) -> (f64, f64, f64, f64) {
    let t5943 = t615 * t5942;
    let t5945 = 0.67745118933333333331e-2_f64 * t1757 * t5943;
    let t5946 = t584 * t1679;
    let t5947 = t1685 * t591;
    let t5948 = t1684 * t5947;
    let t5950 = 0.254044196e-2_f64 * t5946 * t5948;
    let t5951 = t595 * t2065;
    let t5952 = t5951 * t637;
    let t5954 = t595 * t2068;
    (t5945, t5950, t5952, t5954)
}
