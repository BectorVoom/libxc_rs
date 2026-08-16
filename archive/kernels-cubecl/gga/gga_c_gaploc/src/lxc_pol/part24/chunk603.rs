//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 603/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk603<F: Float>(t2508: F, t3420: F, t3216: F, t3226: F, t3218: F, t3223: F, t471: F, t1020: F, t871: F, t3232: F) -> (F, F, F) {
    let t3422 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t3420;
    let t3423 = F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t3216;
    let t3426 = t3226 / F::cast_from(128.0_f64);
    let t3427 = t3423 - F::cast_from(9.0_f64) / F::cast_from(4096.0_f64) * t3218 + F::cast_from(3.0_f64) / F::cast_from(4096.0_f64) * t3223 - t3426;
    let t3428 = t3427 * t471;
    let t3429 = t1020 * t871;
    let t3431 = t3428 + t3429 / F::cast_from(2.0_f64) + t3423 - t3426 - t3232;
    (t3422, t3427, t3431)
}
