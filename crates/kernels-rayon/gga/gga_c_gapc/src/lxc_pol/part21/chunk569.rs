//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 569/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk569(t322: f64, t3307: f64, t1069: f64, t913: f64, t3274: f64, t3276: f64, t3279: f64, t3282: f64, t3286: f64, t3290: f64, t3298: f64, t3301: f64, t3305: f64) -> f64 {
    let t3308 = t3307 * t322;
    let t3310 = t1069 * t913;
    let t3312 = -0.1686740451388888889e-5_f64 * t3274 - 0.84540905957968605066e-5_f64 * t3276 + 0.72463633678258804342e-6_f64 * t3279 + 0.61789714048124642274e-4_f64 * t3282 + 0.14492726735651760868e-5_f64 * t3286 - 0.14492726735651760868e-5_f64 * t3290 + 0.73794894748263888892e-6_f64 * t3298 - 0.25340269868817520617e-4_f64 * t3301 - 0.72463633678258804342e-6_f64 * t3305 + 0.13900948042322754167e-2_f64 * t3308 + 0.13900948042322754167e-2_f64 * t3310;
    t3312
}
