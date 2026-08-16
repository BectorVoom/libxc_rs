//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 673/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk673(t1526: f64, t2640: f64, t9483: f64, t2644: f64, t342: f64, t630: f64, t2680: f64, t683: f64, t191: f64, t7640: f64, t793: f64, t89: f64, t9733: f64) -> (f64, f64, f64, f64, f64) {
    let t10209 = t1526 * t9483 * t2640;
    let t10212 = t342 * t630 * t2644;
    let t10248 = t683 * t2680;
    let t10261 = t191 * t7640;
    let t10279 = t89 * t9733 * t793;
    (t10209, t10212, t10248, t10261, t10279)
}
