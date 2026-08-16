//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 404/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk404(t3630: f64, t738: f64, t270: f64, t3438: f64, t3445: f64, t3604: f64, t3617: f64, t3622: f64, t3627: f64, t1052: f64) -> (f64, f64, f64) {
    let t3631 = t738 * t3630;
    let t3634 = 0.76905262301422242837e-2_f64 * t270 * t3604 + 0.76905262301422242837e-2_f64 * t270 * t3617 + 0.1281754371690370714e-2_f64 * t3438 - 0.23071578690426672851e-1_f64 * t270 * t3622 - 0.1281754371690370714e-2_f64 * t3445 + 0.15381052460284448567e-1_f64 * t270 * t3627 - 0.76905262301422242837e-2_f64 * t270 * t3631;
    let t3638 = t1052 * t1052;
    (t3631, t3634, t3638)
}
