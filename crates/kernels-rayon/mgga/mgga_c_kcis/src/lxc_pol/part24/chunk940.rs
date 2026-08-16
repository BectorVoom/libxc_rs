//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 940/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk940(t19956: f64, t376: f64, t375: f64, t19619: f64, t5176: f64, t5175: f64, t5068: f64, t5172: f64, t1166: f64, t6701: f64, t1817: f64, t5169: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19957 = t376 * t19956;
    let t19958 = t375 * t19957;
    let t19960 = t5176 * t19619;
    let t19961 = t5175 * t19960;
    let t19963 = t5172 * t5068;
    let t19965 = t1166 * t6701;
    let t19967 = t5169 * t1817;
    (t19958, t19960, t19961, t19963, t19965, t19967)
}
