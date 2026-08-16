//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1125/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1125(t14149: f64, t944: f64, t1198: f64, t6854: f64, t2051: f64, t2423: f64, t4063: f64, t1105: f64, t13751: f64, t2494: f64, t3944: f64, t4188: f64, t945: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14150 = t14149 * t944;
    let t14153 = t1198 * t6854;
    let t14154 = t14153 * t2051;
    let t14157 = t4063 * t2423;
    let t14380 = t13751 * t1105;
    let t14383 = t1105 * t944;
    let t14387 = t3944 * t2494;
    let t14390 = t4188 * t945;
    (t14150, t14153, t14154, t14157, t14380, t14383, t14387, t14390)
}
