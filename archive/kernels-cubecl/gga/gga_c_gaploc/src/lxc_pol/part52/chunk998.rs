//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 998/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk998<F: Float>(t41903: F, t46299: F, t46301: F, t46303: F, t46311: F, t46316: F, t46327: F, t46331: F, t46336: F, t46339: F, t46342: F, t46343: F, t46345: F, t46352: F, t46354: F, t46356: F, t46361: F, t46365: F, t46368: F, t47949: F) -> F {
    let t50611 = -t46299 + t46301 + t46303 + t46311 - t46316 + t46327 + F::cast_from(0.57514388930881124514e0_f64) * t46331 + F::cast_from(0.38342925953920749676e1_f64) * t41903 + t46336 - t46339 + t46342 + t46343 - F::cast_from(0.76685851907841499354e0_f64) * t47949 + t46345 + t46352 + t46354 + t46356 - t46361 - t46365 + t46368;
    t50611
}
