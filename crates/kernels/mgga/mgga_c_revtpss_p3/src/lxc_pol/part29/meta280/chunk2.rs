//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1155/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1155<F: Float>(t185: F, t2494: F, t9367: F, t9368: F, t1340: F, t2516: F, t4038: F, t9283: F, t9286: F, t9289: F, t9292: F, t9296: F, t9298: F, t9300: F, t9303: F) -> (F, F, F, F, F) {
    let t9371 = F::new(1.0) / t2494 / t185;
    let t9372 = t9367 * t9368 * t9371;
    let t9374 = F::cast_from(0.10254018858216406658e4_f64) * t1340 * t9372;
    let t9375 = t4038 * t2516;
    let t9385 = -F::cast_from(0.34523333333333333333e1_f64) * t9283 + F::cast_from(0.23015555555555555556e1_f64) * t9286 - F::cast_from(0.26851481481481481482e1_f64) * t9289 - F::cast_from(0.93932222222222222223e0_f64) * t9292 + F::new(0.73355e-1) * t9296 - F::new(0.14671e0) * t9298 - F::cast_from(0.17116166666666666667e0_f64) * t9300 - F::cast_from(0.36793333333333333333e0_f64) * t9303;
    (t9371, t9372, t9374, t9375, t9385)
}
