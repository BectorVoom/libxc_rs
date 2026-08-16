//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 770/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk770(t532: f64, t7933: f64, t1450: f64, t2014: f64, t2034: f64, t5542: f64, t1916: f64, t2042: f64, t1518: f64, t7330: f64, t572: f64, t117: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7934 = t532 * t7933;
    let t7935 = t7934 * t1450;
    let t7936 = t2014 * t7935;
    let t7937 = t2034 * t5542;
    let t7938 = t2014 * t7937;
    let t7949 = 3.0_f64 * t1916 * t2042;
    let t7950 = t7330 * t1518;
    let t7952 = 6.0_f64 * t572 * t7950;
    let t7953 = t117 * t7741;
    (t7934, t7935, t7936, t7937, t7938, t7949, t7950, t7952, t7953)
}
