//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 314/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk314(t2541: f64, t740: f64, t731: f64, t945: f64, t679: f64, t78: f64, t278: f64, t481: f64) -> (f64, f64, f64, f64) {
    let t2542 = t2541 * t740;
    let t2545 = t731 * t945;
    let t2547 = t78 * t679;
    let t2549 = t481 * t2547 * t278;
    (t2542, t2545, t2547, t2549)
}
