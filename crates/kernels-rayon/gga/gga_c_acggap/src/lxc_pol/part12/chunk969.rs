//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 969/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk969(t301: f64, t32262: f64, t694: f64, t1268: f64, t1679: f64, t8040: f64, t10761: f64, t467: f64, t8034: f64, t839: f64, t1674: f64, t922: f64) -> (f64, f64, f64, f64, f64) {
    let t32264 = t694 * t32262 * t301;
    let t32276 = t1679 * t8040 * t1268;
    let t32283 = t1679 * t10761 * t467;
    let t32298 = t694 * t8034 * t839;
    let t32301 = t1674 * t8034 * t922;
    (t32264, t32276, t32283, t32298, t32301)
}
