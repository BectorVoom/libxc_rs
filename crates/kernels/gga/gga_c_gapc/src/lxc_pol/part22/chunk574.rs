//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 574/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk574<F: Float>(t322: F, t3307: F, t1069: F, t913: F, t3274: F, t3276: F, t3279: F, t3282: F, t3286: F, t3290: F, t3298: F, t3301: F, t3305: F) -> (F, F, F) {
    let t3308 = t3307 * t322;
    let t3310 = t1069 * t913;
    let t3312 = -F::cast_from(0.1686740451388888889e-5_f64) * t3274 - F::cast_from(0.84540905957968605066e-5_f64) * t3276 + F::cast_from(0.72463633678258804342e-6_f64) * t3279 + F::cast_from(0.61789714048124642274e-4_f64) * t3282 + F::cast_from(0.14492726735651760868e-5_f64) * t3286 - F::cast_from(0.14492726735651760868e-5_f64) * t3290 + F::cast_from(0.73794894748263888892e-6_f64) * t3298 - F::cast_from(0.25340269868817520617e-4_f64) * t3301 - F::cast_from(0.72463633678258804342e-6_f64) * t3305 + F::cast_from(0.13900948042322754167e-2_f64) * t3308 + F::cast_from(0.13900948042322754167e-2_f64) * t3310;
    (t3308, t3310, t3312)
}
