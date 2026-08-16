//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 790/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk790(t3405: f64, t542: f64, t1013: f64, t8907: f64, t1008: f64, t2057: f64, t2059: f64, t131: f64, t538: f64, t1009: f64, t1010: f64, t1015: f64, t12368: f64, t12371: f64, t12374: f64, t12381: f64, t12385: f64, t1683: f64, t1698: f64, t2001: f64, t3348: f64, t3350: f64, t3356: f64, t3381: f64, t3387: f64, t3392: f64, t3393: f64, t3406: f64, t399: f64) -> f64 {
    let t12392 = t542 * t3405;
    let t12397 = t8907 * t1013;
    let t12401 = t2057 * t1008;
    let t12402 = t12401 * t2059;
    let t12411 = t538 * t131;
    let t12412 = t12411 * t1009;
    let t12417 = -4.0_f64 * t2001 * t12368 - 2.0_f64 * t2001 * t12371 - 4.0_f64 * t12374 * t3356 - 0.38259118126557588605e1_f64 * t3387 * t1683 + 0.38259118126557588605e1_f64 * t1015 * t1683 + 8.0_f64 * t2001 * t3393 * t12381 - 0.2416365355361531912e1_f64 * t12385 * t399 - 0.1208182677680765956e1_f64 * t3406 * t399 - 0.14597053826478655997e1_f64 * t1015 * t1698 + 0.1208182677680765956e1_f64 * t12392 * t399 + 0.29194107652957311994e1_f64 * t3387 * t1698 - 6.0_f64 * t3392 * t12397 * t2059 + 4.0_f64 * t2001 * t12402 - 0.58388215305914623988e1_f64 * t3350 * t1698 + 0.2416365355361531912e1_f64 * t3381 * t399 + 0.29194107652957311994e1_f64 * t1010 * t1698 - 0.2416365355361531912e1_f64 * t12412 * t399 + 0.2416365355361531912e1_f64 * t3348 * t399;
    t12417
}
