//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1285/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1285(t14101: f64, t8837: f64, t4028: f64, t9098: f64, t14079: f64, t3283: f64, t4049: f64, t9594: f64, t1154: f64, t51387: f64, t1184: f64, t8975: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54297 = t14101 * t8837;
    let t54299 = t4028 * t9098;
    let t54301 = t14079 * t3283;
    let t54303 = t4049 * t9594;
    let t54305 = t51387 * t1154;
    let t54307 = t1184 * t8975;
    (t54297, t54299, t54301, t54303, t54305, t54307)
}
