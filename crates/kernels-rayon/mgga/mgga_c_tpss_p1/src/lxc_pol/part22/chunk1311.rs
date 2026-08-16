//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1311/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1311(t236: f64, t339: f64, t61038: f64, t10782: f64, t10786: f64, t19703: f64, t10809: f64, t17964: f64, t3671: f64, t61033: f64, t10602: f64, t10575: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t63920 = t339 * t61038 * t236;
    let t63921 = t63920 * t10782;
    let t63923 = t19703 * t10786;
    let t63925 = t17964 * t10809;
    let t63928 = t61033 * t3671;
    let t63930 = t17964 * t10602;
    let t63932 = t17964 * t10575;
    (t63921, t63923, t63925, t63928, t63930, t63932)
}
