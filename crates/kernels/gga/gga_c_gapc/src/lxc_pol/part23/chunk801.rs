//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 801/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk801<F: Float>(t3094: F, t9359: F, t5541: F, t612: F, t1671: F, t5544: F, t9326: F, t9331: F, t9334: F, t9337: F, t9339: F, t9341: F, t9344: F, t9346: F, t9349: F, t9351: F, t9354: F, t9357: F) -> F {
    let t9360 = t3094 * t9359;
    let t9362 = t5541 * t612;
    let t9363 = t1671 * t5544;
    let t9364 = t9362 * t9363;
    let t9366 = -F::new(0.11795371371935910947e-5) * t9326 - F::new(0.36954560225358884233e-5) * t9331 + F::new(0.7588373973867992891e-7) * t9334 - F::new(0.13492128925537291361e-6) * t9337 - F::new(0.15176747947735985782e-6) * t9339 + F::new(0.26984257851074582721e-6) * t9341 + F::new(0.4637672555408563478e-4) * t9344 - F::new(0.4637672555408563478e-4) * t9346 - F::new(0.86880925264517213544e-4) * t9349 - F::new(0.17376185052903442709e-3) * t9351 + F::new(0.14480154210752868924e-5) * t9354 - F::new(0.86880925264517213544e-4) * t9357 + F::new(0.14480154210752868924e-5) * t9360 + F::new(0.50680539737635041234e-4) * t9364;
    t9366
}
