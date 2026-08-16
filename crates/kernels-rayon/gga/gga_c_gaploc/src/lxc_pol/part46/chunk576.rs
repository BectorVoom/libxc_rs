//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 576/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk576(t10027: f64, t10065: f64, t10102: f64, t9827: f64, t9867: f64, t9907: f64, t9938: f64, t9980: f64, t2440: f64, t988: f64, t2268: f64, t2756: f64, t894: f64) -> (f64, f64, f64) {
    let t10105 = t9827 + t9867 + t9907 + t9938 + t9980 + t10027 + t10065 + t10102;
    let t10113 = t2440 * t988;
    let t10115 = 0.28455006635676149599e-1_f64 * t2268 * t10113;
    let t10116 = t894 * t2756;
    (t10105, t10115, t10116)
}
