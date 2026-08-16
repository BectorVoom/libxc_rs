//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 604/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk604(t422: f64, t423: f64, t5679: f64, t1008: f64, t1886: f64, t1891: f64, t174: f64, t5506: f64, t387: f64, t1849: f64, t301: f64) -> (f64, f64, f64, f64, f64) {
    let t5681 = t422 * t5679 * t423;
    let t5684 = t1008 * t1886;
    let t5686 = t1008 * t1891;
    let t5688 = t174 * t5506;
    let t5690 = t422 * t387 * t5688;
    let t5693 = t1849 * t301;
    (t5681, t5684, t5686, t5690, t5693)
}
