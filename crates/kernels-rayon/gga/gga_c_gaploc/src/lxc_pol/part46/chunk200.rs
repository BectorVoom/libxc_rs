//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 200/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk200(t958: f64, t959: f64, t531: f64, t948: f64, t808: f64, t935: f64, t568: f64, t325: f64, t911: f64) -> (f64, f64, f64, f64, f64) {
    let t960 = t958 * t959;
    let t962 = t531 * t948;
    let t965 = t808 * t935;
    let t966 = t568 * t965;
    let t969 = t911 * t325;
    (t960, t962, t965, t966, t969)
}
