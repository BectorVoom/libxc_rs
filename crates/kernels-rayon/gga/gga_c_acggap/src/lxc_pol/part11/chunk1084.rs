//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1084/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1084(t1072: f64, t535: f64, t7507: f64, t7512: f64, t1131: f64, t2060: f64, t2288: f64, t8927: f64, t2297: f64, t4256: f64, t7450: f64, t839: f64) -> (f64, f64, f64) {
    let t34879 = t7507 * t7512 * t535 * t1072;
    let t34883 = t2060 * t8927 * t2288 * t1131;
    let t34887 = t7450 * t4256 * t2297 * t839;
    (t34879, t34883, t34887)
}
