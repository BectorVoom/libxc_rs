//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1180/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1180(t3748: f64, t3975: f64, t3972: f64, t13544: f64, t13776: f64, t12213: f64, t2409: f64, t4164: f64, t3744: f64, t3959: f64, t3809: f64, t1178: f64, t371: f64, t3896: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15186 = t3975 * t3748;
    let t15187 = t3972 * t15186;
    let t15191 = t3975 * t13544;
    let t15192 = t13776 * t15191;
    let t15195 = t2409 * t12213 * t4164;
    let t15198 = t3959 * t3744;
    let t15200 = t3975 * t3809;
    let t15201 = t3972 * t15200;
    let t15204 = t371 * t1178 * t3896;
    (t15186, t15187, t15191, t15192, t15195, t15198, t15200, t15201, t15204)
}
