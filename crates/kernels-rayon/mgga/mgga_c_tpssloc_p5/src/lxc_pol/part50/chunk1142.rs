//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1142/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1142(t2752: f64, t30752: f64, t10143: f64, t8365: f64, t193: f64, t201: f64, t8369: f64, t1054: f64, t6815: f64, t23384: f64, t30862: f64, t1921: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t113111 = t30752 * t2752;
    let t113117 = t8365 * t10143;
    let t113131 = t193 * t201 * t8365;
    let t113135 = t193 * t201 * t8369;
    let t113149 = t1054 * t6815;
    let t113177 = t23384 * t30862;
    let t113201 = t1921 * t113149;
    (t113111, t113117, t113131, t113135, t113149, t113177, t113201)
}
