//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 754/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk754(t10194: f64, t113: f64, t2956: f64, t909: f64, t4381: f64, t2252: f64, t342: f64, t784: f64, t1526: f64, t2640: f64, t9483: f64, t2644: f64, t630: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10195 = t10194 * t113;
    let t10198 = t2956 * t909;
    let t10199 = t10198 * t4381;
    let t10207 = t342 * t2252 * t784 / 18.0_f64;
    let t10209 = t1526 * t9483 * t2640;
    let t10212 = t342 * t630 * t2644;
    (t10195, t10198, t10199, t10207, t10209, t10212)
}
