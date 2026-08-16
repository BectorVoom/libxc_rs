//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 887/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk887(t10383: f64, t339: f64, t3069: f64, t3180: f64, t3036: f64, t67: f64, t3067: f64, t3186: f64, t3062: f64, t820: f64, t3200: f64, t3051: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10385 = 5.0_f64 / 1296.0_f64 * t339 * t10383;
    let t10390 = t3180 * t3069;
    let t10401 = t3036 * t67;
    let t10402 = t3067 * t10401;
    let t10403 = t3186 * t10402;
    let t10408 = t820 * t3062;
    let t10413 = t3200 * t10402;
    let t10422 = t820 * t3051;
    (t10385, t10390, t10401, t10403, t10408, t10413, t10422)
}
