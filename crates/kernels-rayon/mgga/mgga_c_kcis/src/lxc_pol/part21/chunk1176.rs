//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1176/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1176(t3609: f64, t3643: f64, t11228: f64, t433: f64, t436: f64, t782: f64, t9266: f64, t142: f64, t164: f64, t9273: f64, t113: f64, t8750: f64) -> (f64, f64, f64, f64, f64) {
    let t35576 = t3609 * t3643;
    let t35615 = t433 / t11228 / t436;
    let t35630 = t9266 * t782;
    let t35635 = t142 / t9273 / t164;
    let t36222 = t113 * t8750;
    (t35576, t35615, t35630, t35635, t36222)
}
