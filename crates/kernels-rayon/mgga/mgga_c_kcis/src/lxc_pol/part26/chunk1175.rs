//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1175/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1175(t779: f64, t9274: f64, t2531: f64, t2537: f64, t782: f64, t9266: f64, t142: f64, t164: f64, t9273: f64, t113: f64, t8750: f64, t898: f64, t9005: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31271 = t779 * t9274;
    let t31274 = t2531 * t2537;
    let t35630 = t9266 * t782;
    let t35635 = t142 / t9273 / t164;
    let t36222 = t113 * t8750;
    let t36429 = t9005 * t898;
    (t31271, t31274, t35630, t35635, t36222, t36429)
}
