//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1191/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1191(t14091: f64, t51329: f64, t14028: f64, t2273: f64, t2209: f64, t4021: f64, t6562: f64, t2196: f64, t3065: f64, t14046: f64, t2173: f64, t3969: f64, t916: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51330 = t14091 * t51329;
    let t51332 = t14028 * t2273;
    let t51334 = t4021 * t2209;
    let t51335 = t51334 * t6562;
    let t51338 = t3065 * t2196;
    let t51341 = t14046 * t2173;
    let t51350 = t3969 * t916;
    (t51330, t51332, t51335, t51338, t51341, t51350)
}
