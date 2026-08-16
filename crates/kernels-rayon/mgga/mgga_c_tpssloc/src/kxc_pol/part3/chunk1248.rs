//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1248/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1248(t1385: f64, t5353: f64, t3887: f64, t16413: f64, t539: f64, t225: f64, t5217: f64, t1834: f64, t3752: f64, t1323: f64, t5318: f64, t16122: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16452 = t5353 * t1385;
    let t16453 = t3887 * t16452;
    let t16458 = t539 * t16413;
    let t16460 = t5217 * t225;
    let t16463 = t3752 * t1834;
    let t16465 = t1323 * t5318;
    let t16468 = t16122 * t562;
    (t16453, t16458, t16460, t16463, t16465, t16468)
}
