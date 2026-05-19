//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1384/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1384<F: Float>(t33724: F, t33730: F, t43414: F, t44193: F, t44198: F, t52389: F, t52391: F, t52393: F, t58348: F, t58352: F, t58356: F, t58360: F, t58363: F, t58367: F) -> F {
    let t58701 = -F::cast_from(0.18396666666666666667e0_f64) * t44193 + F::new(0.11038e1) * t44198 - F::cast_from(0.53675555555555555556e0_f64) * t43414 + F::cast_from(0.12524296296296296297e1_f64) * t33724 + F::cast_from(0.98115555555555555556e0_f64) * t33730 + F::new(0.24154e1) * t58348 + F::new(0.99342e0) * t58352 - F::new(0.22076e0) * t58356 - F::new(0.298026e1) * t58360 + F::new(0.66228e0) * t58363 - F::new(0.11038e0) * t58367 + F::cast_from(0.40256666666666666668e0_f64) * t52389 + F::new(0.24154e1) * t52391 + F::cast_from(0.44729629629629629629e0_f64) * t52393;
    t58701
}
