//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 598/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk598<F: Float>(t3439: F, t829: F, t3438: F, t311: F, t896: F, t315: F, t3171: F, t3398: F, t3400: F, t3409: F, t3416: F, t3419: F, t3422: F, t3425: F, t3428: F, t3432: F, t3435: F) -> (F, F, F, F, F, F) {
    let t3440 = t829 * t3439;
    let t3441 = t3438 * t3440;
    let t3443 = t311 * t896;
    let t3444 = t3171 * t315;
    let t3445 = t3443 * t3444;
    let t3447 = -F::new(0.13900948042322754167e-2) * t3398 + F::new(0.54106179813099907243e-4) * t3400 + F::new(0.42205124476153752644e-7) * t3409 + F::new(0.84410248952307505288e-7) * t3416 - F::new(0.37941869869339964455e-7) * t3419 + F::new(0.67460644627686456803e-7) * t3422 - F::new(0.86880925264517213544e-4) * t3425 - F::new(0.86880925264517213544e-4) * t3428 + F::new(0.7240077105376434462e-6) * t3432 - F::new(0.11594181388521408695e-4) * t3435 + F::new(0.14068374825384584215e-8) * t3441 - F::new(0.34752370105806885418e-4) * t3445;
    (t3440, t3441, t3443, t3444, t3445, t3447)
}
