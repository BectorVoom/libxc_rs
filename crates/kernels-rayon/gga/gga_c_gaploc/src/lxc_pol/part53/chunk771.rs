//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 771/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk771(t16692: f64, t201: f64, t2536: f64, t2925: f64, t1022: f64, t7275: f64, t10938: f64, t2021: f64, t10007: f64, t10627: f64, t32435: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33332 = t201 * t16692;
    let t33348 = t2536 * t2925;
    let t33360 = t7275 * t1022;
    let t33565 = t2021 * t10938;
    let t33601 = t10007 * t10627;
    let t33680 = t739 * t32435;
    (t33332, t33348, t33360, t33565, t33601, t33680)
}
