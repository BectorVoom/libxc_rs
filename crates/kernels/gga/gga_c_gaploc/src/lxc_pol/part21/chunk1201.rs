//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1201/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1201<F: Float>(t10643: F, t7137: F, t2508: F, t7226: F, t7291: F, t8483: F, t21446: F, t3009: F, t21783: F, t27837: F, t27840: F, t27844: F, t27848: F, t27853: F, t27856: F, t27858: F, t27860: F, t471: F) -> (F, F, F, F, F) {
    let t32277 = F::cast_from(0.14355648962932151996e0_f64) * t7137 * t10643;
    let t32281 = F::cast_from(0.92286314761706691402e-1_f64) * t2508 * t7226 * t8483 * t7291;
    let t32285 = F::cast_from(0.92286314761706691402e-1_f64) * t2508 * t7226 * t3009 * t21446;
    let t32289 = F::cast_from(0.46143157380853345701e-1_f64) * t2508 * t7226 * t3009 * t21783;
    let t32300 = (F::new(189.0) / F::new(512.0) * t27837 - F::new(2499.0) / F::new(16384.0) * t27840 + F::new(1239.0) / F::new(524288.0) * t27844 - F::new(441.0) / F::new(0.16777216e8) * t27848 + F::new(147.0) / F::new(0.16777216e8) * t27853 - F::new(413.0) / F::new(524288.0) * t27856 + F::new(833.0) / F::new(16384.0) * t27858 - F::new(63.0) / F::new(512.0) * t27860) * t471;
    (t32277, t32281, t32285, t32289, t32300)
}
