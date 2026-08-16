//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1886/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1886(t7289: f64, t96282: f64, t26277: f64, t94776: f64, t25950: f64, t26292: f64, t26230: f64, t94764: f64, t94768: f64, t94763: f64, t26234: f64, t94890: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96284 = 0.39982213492741449076e-1_f64 * t7289 * t96282;
    let t96287 = t94776 * t26277;
    let t96289 = t25950 * t26292;
    let t96291 = t26230 * t94764;
    let t96292 = t94768 * t96291;
    let t96294 = t94763 * t96291;
    let t96296 = t94890 * t26234;
    (t96284, t96287, t96289, t96292, t96294, t96296)
}
