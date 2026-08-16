//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1123/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1123(t14058: f64, t935: f64, t4021: f64, t885: f64, t2149: f64, t3065: f64, t876: f64, t1189: f64, t2334: f64, t2153: f64, t899: f64, t922: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14059 = t14058 * t935;
    let t14063 = t4021 * t885;
    let t14064 = t14063 * t2149;
    let t14069 = t3065 * t876;
    let t14072 = t1189 * t2334;
    let t14073 = 119.0_f64 / 6912.0_f64 * t14072;
    let t14079 = t899 * t2153 * t922;
    (t14059, t14063, t14064, t14069, t14073, t14079)
}
