//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 515/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk515(t2465: f64, t2571: f64, t2464: f64, t825: f64, t1645: f64, t9740: f64, t2194: f64, t3308: f64, t7068: f64, t883: f64, t1967: f64, t7810: f64) -> (f64, f64, f64, f64, f64) {
    let t9850 = t2465 * t2571;
    let t9851 = t2464 * t9850;
    let t9852 = t825 * t9851;
    let t9858 = t1645 * t9740;
    let t9873 = t2194 * t3308;
    let t9889 = t883 * t7068;
    let t9890 = t1967 * t9889;
    let t9891 = t7810 * t9890;
    (t9852, t9858, t9873, t9889, t9891)
}
