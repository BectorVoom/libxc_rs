//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 895/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk895(t1601: f64, t167: f64, t6163: f64, t2105: f64, t25: f64, t1599: f64, t2104: f64, t531: f64, t833: f64, t4440: f64, t286: f64, t494: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6164 = t1601 * t167;
    let t6165 = t6163 * t6164;
    let t6168 = t25 * t2105;
    let t6169 = t1599 * t6168;
    let t6171 = t2104 * t531;
    let t6172 = t6171 * t833;
    let t6173 = t4440 * t6172;
    let t6176 = t286 * t494;
    (t6164, t6165, t6169, t6171, t6172, t6173, t6176)
}
