//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1239/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1239(t3038: f64, t3972: f64, t3975: f64, t9520: f64, t1176: f64, t14639: f64, t6365: f64, t923: f64, t3959: f64, t8766: f64, t1113: f64, t28647: f64) -> (f64, f64, f64, f64) {
    let t53395 = t3972 * t3975 * t3038 * t9520;
    let t53424 = t1176 * t923 * t6365 * t14639;
    let t53426 = t3959 * t8766;
    let t53432 = t3972 * t3975 * t1113 * t28647;
    (t53395, t53424, t53426, t53432)
}
