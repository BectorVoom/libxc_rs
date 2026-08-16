//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1166/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1166(t1193: f64, t3200: f64, t338: f64, t14001: f64, t4130: f64, t1192: f64, t3306: f64, t2409: f64, t3067: f64, t13953: f64, t4135: f64, t3294: f64, t3975: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14742 = t338 * t3200 * t1193;
    let t14745 = t14001 * t4130;
    let t14747 = t1192 * t3306;
    let t14749 = t2409 * t3067 * t14747;
    let t14752 = t13953 * t4135;
    let t14754 = t3975 * t3294;
    (t14742, t14745, t14747, t14749, t14752, t14754)
}
