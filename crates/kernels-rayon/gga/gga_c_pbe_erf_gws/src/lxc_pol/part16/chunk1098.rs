//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1098/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1098(t14046: f64, t4029: f64, t3139: f64, t6178: f64, t4028: f64, t1184: f64, t2212: f64, t2302: f64, t4049: f64, t864: f64, t899: f64, t922: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14047 = t14046 * t4029;
    let t14048 = 7.0_f64 / 144.0_f64 * t14047;
    let t14049 = t3139 * t6178;
    let t14050 = t4028 * t14049;
    let t14052 = t1184 * t2212;
    let t14055 = t4049 * t2302;
    let t14058 = t899 * t864 * t922;
    (t14047, t14048, t14049, t14050, t14052, t14055, t14058)
}
