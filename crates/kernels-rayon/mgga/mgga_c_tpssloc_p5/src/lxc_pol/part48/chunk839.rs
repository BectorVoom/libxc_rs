//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 839/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk839(t1998: f64, t6955: f64, t214: f64, t1985: f64, t2314: f64, t8326: f64, t5113: f64, t3938: f64, t671: f64, t3941: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31206 = t1998 * t6955;
    let t31207 = t214 * t31206;
    let t31209 = 0.16449340668482264365e-1_f64 * t1985 * t31207;
    let t31236 = t2314 * t8326;
    let t31237 = 2.0_f64 * t31236;
    let t31238 = t5113 * t8326;
    let t31239 = 2.0_f64 * t31238;
    let t31283 = t3938 * t8326;
    let t31284 = 0.135e2_f64 * t31283;
    let t31285 = t8326 * t671;
    let t31286 = t3941 * t31285;
    let t31287 = 27.0_f64 * t31286;
    (t31206, t31207, t31209, t31236, t31237, t31238, t31239, t31283, t31284, t31285, t31286, t31287)
}
