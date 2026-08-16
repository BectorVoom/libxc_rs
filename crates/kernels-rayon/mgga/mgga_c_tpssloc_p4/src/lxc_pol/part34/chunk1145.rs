//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1145/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1145(t5544: f64, t857: f64, t23164: f64, t23204: f64, t28276: f64, t28342: f64, t81979: f64, t252: f64, t5527: f64, t28333: f64, t6562: f64, t794: f64) -> (f64, f64, f64, f64, f64) {
    let t98253 = t857 * t5544;
    let t98322 = t23164 * t23204 * t28276;
    let t98330 = t81979 * t28342;
    let t98336 = t252 * t5527;
    let t98342 = t6562 * t794 * t28333;
    (t98253, t98322, t98330, t98336, t98342)
}
