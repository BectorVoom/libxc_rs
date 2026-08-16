//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1211/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1211(t898: f64, t1178: f64, t13918: f64, t2332: f64, t864: f64, t899: f64, t907: f64, t13806: f64, t915: f64, t2276: f64, t1477: f64, t345: f64, t56: f64, t859: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t51020 = t898 * param_a_c;
    let t51021 = t1178 * t51020;
    let t51066 = t1178 * t13918;
    let t51200 = t899 * t864 * t2332;
    let t51201 = t51200 * t907;
    let t51213 = t13806 * t915;
    let t51214 = t2276 * t51213;
    let t51221 = t345 * t1477 * t56 * t859;
    (t51020, t51021, t51066, t51200, t51201, t51213, t51214, t51221)
}
