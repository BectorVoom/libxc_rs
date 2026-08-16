//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 844/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk844(t1022: f64, t7275: f64, t1: f64, t32364: f64, t787: f64, t10938: f64, t2021: f64, t33137: f64, t10007: f64, t10627: f64, t32435: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33360 = t7275 * t1022;
    let t33399 = t787 * t32364 * t1;
    let t33565 = t2021 * t10938;
    let t33575 = t33137 * t1;
    let t33576 = t2021 * t33575;
    let t33601 = t10007 * t10627;
    let t33680 = t739 * t32435;
    (t33360, t33399, t33565, t33575, t33576, t33601, t33680)
}
