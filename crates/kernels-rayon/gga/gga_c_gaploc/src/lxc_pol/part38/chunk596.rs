//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 596/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk596(t10847: f64, t7573: f64, t7572: f64, t10820: f64, t326: f64, t825: f64, t7585: f64, t2684: f64, t1: f64, t2084: f64, t106: f64, t787: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10903 = t7573 * t10847;
    let t10905 = 0.69017266717057349418e1_f64 * t7572 * t10903;
    let t10906 = t326 * t10820;
    let t10908 = 0.92023022289409799224e1_f64 * t825 * t10906;
    let t10909 = t7585 * t10820;
    let t10911 = 0.43710935587469654631e2_f64 * t2684 * t10909;
    let t10912 = t2084 * t1;
    let t10913 = t10912 * t106;
    let t10914 = t787 * t10913;
    (t10905, t10908, t10911, t10912, t10913, t10914)
}
