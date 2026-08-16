//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1202/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1202(t2118: f64, t838: f64, t14138: f64, t822: f64, t2232: f64, t4386: f64, t13872: f64, t13953: f64, t13972: f64, t14118: f64, t13899: f64, t3979: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51717 = t2118 * t838;
    let t51719 = t822 * t51717 * t14138;
    let t51721 = t4386 * t2232;
    let t51724 = t13953 * t13872;
    let t51771 = t13972 * t14118;
    let t51807 = t3979 * t13899;
    (t51717, t51719, t51721, t51724, t51771, t51807)
}
