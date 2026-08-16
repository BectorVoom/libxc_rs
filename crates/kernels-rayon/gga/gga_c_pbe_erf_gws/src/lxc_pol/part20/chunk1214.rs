//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1214/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1214(t898: f64, t1178: f64, t1193: f64, t353: f64, t745: f64, t859: f64, t13918: f64, t2416: f64, t4052: f64, t2242: f64, t4013: f64, t22509: f64, t4018: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51020 = t898 * param_a_c;
    let t51021 = t1178 * t51020;
    let t51053 = t353 * t1193 * t745;
    let t51054 = t859 * t51053;
    let t51066 = t1178 * t13918;
    let t51084 = t2416 * t4052;
    let t51156 = t2242 * t4013;
    let t51168 = t22509 * t4018;
    (t51020, t51021, t51054, t51066, t51084, t51156, t51168)
}
