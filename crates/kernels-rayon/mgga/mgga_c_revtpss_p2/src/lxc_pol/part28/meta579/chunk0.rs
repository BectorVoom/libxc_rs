//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2043/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2043(t25516: f64, t3278: f64, t25586: f64, t342: f64, t994: f64, t11223: f64, t1976: f64, t27639: f64, t995: f64, t25610: f64, t3043: f64, t25604: f64, t7156: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93821 = t3278 * t25516;
    let t93867 = t342 * t25586;
    let t93881 = t994 * t25586;
    let t93884 = t11223 * t1976;
    let t93890 = t995 * t27639;
    let t93897 = t25610 * t27639;
    let t93901 = t3043 * t1976;
    let t93904 = t7156 * t25604;
    (t93821, t93867, t93881, t93884, t93890, t93897, t93901, t93904)
}
