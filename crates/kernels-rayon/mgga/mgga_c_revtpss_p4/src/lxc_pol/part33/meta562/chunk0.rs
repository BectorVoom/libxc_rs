//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1959/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1959(t2042: f64, t6941: f64, t1916: f64, t7950: f64, t7953: f64, t1936: f64, t5883: f64, t572: f64, t1518: f64, t28276: f64, t5920: f64, t7330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30180 = 3.0_f64 * t6941 * t2042;
    let t30182 = 12.0_f64 * t1916 * t7950;
    let t30184 = 6.0_f64 * t1916 * t7953;
    let t30185 = t5883 * t1936;
    let t30187 = 6.0_f64 * t572 * t30185;
    let t30188 = t28276 * t1518;
    let t30190 = 12.0_f64 * t572 * t30188;
    let t30191 = t7330 * t5920;
    (t30180, t30182, t30184, t30185, t30187, t30188, t30190, t30191)
}
