//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1329/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1329(t34716: f64, t6576: f64, t7047: f64, t993: f64, t1540: f64, t196: f64, t20157: f64, t3176: f64, t4525: f64, t8124: f64, t1560: f64, t31775: f64) -> (f64, f64, f64, f64) {
    let t34717 = 0.38342925953920749676e0_f64 * t34716;
    let t34719 = t6576 * t993 * t7047;
    let t34720 = 0.19171462976960374838e0_f64 * t34719;
    let t34726 = 0.12269736305254639897e2_f64 * t196 * t4525 * t20157 * t8124 * t3176 * t1540;
    let t34730 = 0.27606906686822939768e2_f64 * t196 * t1560 * t20157 * t31775;
    (t34717, t34720, t34726, t34730)
}
