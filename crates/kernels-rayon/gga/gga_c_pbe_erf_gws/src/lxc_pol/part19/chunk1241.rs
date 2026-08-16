//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1241/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1241(t4026: f64, t863: f64, t885: f64, t828: f64, t3287: f64, t51255: f64, t3142: f64, t51382: f64, t1125: f64, t51292: f64, t14024: f64, t3120: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54244 = t863 * t4026 * t885;
    let t54253 = t4026 * t828;
    let t54257 = t51255 * t3287;
    let t54259 = t51382 * t3142;
    let t54267 = t1125 * t51292;
    let t54271 = t3120 * t14024;
    (t54244, t54253, t54257, t54259, t54267, t54271)
}
