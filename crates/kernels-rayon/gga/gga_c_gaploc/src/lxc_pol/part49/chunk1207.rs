//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1207/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1207(t1457: f64, t46915: f64, t557: f64, t1572: f64, t46920: f64, t42380: f64, t42381: f64, t42385: f64, t42388: f64, t42390: f64, t42392: f64, t48167: f64, t48172: f64, t48175: f64, t48178: f64) -> f64 {
    let t48182 = 0.10725146985555128001e1_f64 * t557 * t1457 * t46915;
    let t48185 = 0.71500979903700853338e0_f64 * t1572 * t1457 * t46920;
    let t48186 = -0.35750489951850426669e0_f64 * t48167 + 0.42900587942220512003e1_f64 * t48172 - 0.11502877786176224903e2_f64 * t48175 - 0.19171462976960374838e0_f64 * t48178 - t48182 + t48185 - t42380 + t42381 - t42385 + t42388 - t42390 + t42392;
    t48186
}
