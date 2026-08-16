//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1966/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1966(t29991: f64, t30159: f64, t3: f64, t2042: f64, t6941: f64, t1916: f64, t7950: f64, t7953: f64, t1936: f64, t5883: f64, t572: f64, t1518: f64, t28276: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30160 = t29991 + t30159;
    let t30161 = t3 * t30160;
    let t30171 = param_d * t30160;
    let t30180 = 3.0_f64 * t6941 * t2042;
    let t30182 = 12.0_f64 * t1916 * t7950;
    let t30184 = 6.0_f64 * t1916 * t7953;
    let t30185 = t5883 * t1936;
    let t30187 = 6.0_f64 * t572 * t30185;
    let t30188 = t28276 * t1518;
    (t30160, t30161, t30171, t30180, t30182, t30184, t30185, t30187, t30188)
}
