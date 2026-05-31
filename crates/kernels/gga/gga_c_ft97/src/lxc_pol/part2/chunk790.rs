//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 790/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk790<F: Float>(t3405: F, t542: F, t1013: F, t8907: F, t1008: F, t2057: F, t2059: F, t131: F, t538: F, t1009: F, t1010: F, t1015: F, t12368: F, t12371: F, t12374: F, t12381: F, t12385: F, t1683: F, t1698: F, t2001: F, t3348: F, t3350: F, t3356: F, t3381: F, t3387: F, t3392: F, t3393: F, t3406: F, t399: F) -> F {
    let t12392 = t542 * t3405;
    let t12397 = t8907 * t1013;
    let t12401 = t2057 * t1008;
    let t12402 = t12401 * t2059;
    let t12411 = t538 * t131;
    let t12412 = t12411 * t1009;
    let t12417 = -F::cast_from(4.0_f64) * t2001 * t12368 - F::cast_from(2.0_f64) * t2001 * t12371 - F::cast_from(4.0_f64) * t12374 * t3356 - F::cast_from(0.38259118126557588605e1_f64) * t3387 * t1683 + F::cast_from(0.38259118126557588605e1_f64) * t1015 * t1683 + F::cast_from(8.0_f64) * t2001 * t3393 * t12381 - F::cast_from(0.2416365355361531912e1_f64) * t12385 * t399 - F::cast_from(0.1208182677680765956e1_f64) * t3406 * t399 - F::cast_from(0.14597053826478655997e1_f64) * t1015 * t1698 + F::cast_from(0.1208182677680765956e1_f64) * t12392 * t399 + F::cast_from(0.29194107652957311994e1_f64) * t3387 * t1698 - F::cast_from(6.0_f64) * t3392 * t12397 * t2059 + F::cast_from(4.0_f64) * t2001 * t12402 - F::cast_from(0.58388215305914623988e1_f64) * t3350 * t1698 + F::cast_from(0.2416365355361531912e1_f64) * t3381 * t399 + F::cast_from(0.29194107652957311994e1_f64) * t1010 * t1698 - F::cast_from(0.2416365355361531912e1_f64) * t12412 * t399 + F::cast_from(0.2416365355361531912e1_f64) * t3348 * t399;
    t12417
}
