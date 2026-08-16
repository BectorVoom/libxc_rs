//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1286/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1286(t25586: f64, t342: f64, t11627: f64, t1976: f64, t994: f64, t11223: f64, t27639: f64, t995: f64, t3151: f64, t3153: f64, t19482: f64, t988: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93867 = t342 * t25586;
    let t93870 = t11627 * t1976;
    let t93881 = t994 * t25586;
    let t93884 = t11223 * t1976;
    let t93890 = t995 * t27639;
    let t93892 = t1976 * t3151 * t3153;
    let t93893 = t19482 * t988;
    (t93867, t93870, t93881, t93884, t93890, t93892, t93893)
}
