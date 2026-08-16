//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 766/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk766(t1916: f64, t2042: f64, t1518: f64, t7330: f64, t572: f64, t117: f64, t7741: f64, t1918: f64, t2040: f64, t573: f64, t7944: f64, t2047: f64, t7719: f64) -> (f64, f64, f64, f64) {
    let t7949 = 3.0_f64 * t1916 * t2042;
    let t7950 = t7330 * t1518;
    let t7952 = 6.0_f64 * t572 * t7950;
    let t7953 = t117 * t7741;
    let t7955 = 3.0_f64 * t572 * t7953;
    let t7956 = 3.0_f64 * t1918 * t2040 + t573 * t7944 + t7949 + t7952 + t7955;
    let t7964 = t2047 * t7719;
    (t7950, t7953, t7956, t7964)
}
