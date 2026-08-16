//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 991/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk991(t19345: f64, t296: f64, t4969: f64, t835: f64, t882: f64, t1255: f64, t2862: f64, t4162: f64, t4167: f64, t4246: f64, t840: f64, t5299: f64, t824: f64) -> (f64, f64, f64, f64, f64) {
    let t19346 = t296 * t19345;
    let t19351 = t835 * t882 * t4969;
    let t19355 = t2862 * t1255 * t4162;
    let t19359 = t840 * t4246 * t4167;
    let t19362 = t5299 * t824;
    (t19346, t19351, t19355, t19359, t19362)
}
