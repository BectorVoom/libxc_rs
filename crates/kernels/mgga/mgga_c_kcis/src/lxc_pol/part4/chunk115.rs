//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 115/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk115<F: Float>(t305: F, t317: F, t318: F, t319: F, t323: F, t324: F, t325: F, t331: F, t333: F, t334: F, t335: F) -> F {
    let t339 = -F::cast_from(0.11955719325063177623e-1_f64) * t305 + F::new(0.263475e-2) * t317 * t318 * t319 - F::new(0.4755e-3) * t323 * t324 * t325 + F::cast_from(0.2589769453898153438e-4_f64) * t331 - F::new(0.21605625e-5) * t333 * t334 * t335;
    t339
}
