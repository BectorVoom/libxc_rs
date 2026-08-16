//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 573/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk573(t29: f64, t32: f64, t8991: f64, t23: f64, t7368: f64, t143: f64, t7763: f64, t1642: f64, t525: f64, t7800: f64, t10: f64, t144: f64, t3050: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8994 = t8991 / t32 / t29;
    let t9016 = t23 * t7368;
    let t9025 = t143 * t7763;
    let t9049 = t1642 * t525;
    let t9054 = t143 * t7800;
    let t9071 = t10 * t3050 * t144;
    (t8994, t9016, t9025, t9049, t9054, t9071)
}
