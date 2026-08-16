//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1066/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1066(t2728: f64, t3459: f64, t5559: f64, t10800: f64, t1960: f64, t3511: f64, t11125: f64, t977: f64, t2595: f64, t33992: f64, t13241: f64, t841: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44207 = 12.0_f64 * t5559 * t3459 * t2728;
    let t44208 = t10800 * t2728;
    let t44211 = t1960 * t3511 * t2728;
    let t44215 = t1960 * t11125 * t977;
    let t44217 = t33992 * t2595;
    let t44221 = 6.0_f64 * t5559 * t13241 * t841;
    (t44207, t44208, t44211, t44215, t44217, t44221)
}
