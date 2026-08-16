//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 621/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk621(t10898: f64, t8516: f64, t959: f64, t10847: f64, t7573: f64, t7572: f64, t10820: f64, t326: f64, t825: f64, t7585: f64, t2684: f64, t1: f64, t2084: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10899 = 0.42603251059911944084e-1_f64 * t10898;
    let t10900 = t8516 * t959;
    let t10901 = 0.14896037479937677779e-1_f64 * t10900;
    let t10903 = t7573 * t10847;
    let t10905 = 0.69017266717057349418e1_f64 * t7572 * t10903;
    let t10906 = t326 * t10820;
    let t10908 = 0.92023022289409799224e1_f64 * t825 * t10906;
    let t10909 = t7585 * t10820;
    let t10911 = 0.43710935587469654631e2_f64 * t2684 * t10909;
    let t10912 = t2084 * t1;
    (t10899, t10901, t10905, t10908, t10911, t10912)
}
