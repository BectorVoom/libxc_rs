//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1301/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1301(t31: f64, t625: f64, t31864: f64, t607: f64, t8308: f64, t31857: f64, t32344: f64, t31868: f64, t240: f64, t8307: f64, t8513: f64, t8663: f64) -> (f64, f64, f64, f64, f64) {
    let t117496 = t625 * t31;
    let t117499 = t31864 * t8308 * t117496 * t607;
    let t117516 = t31857 * t32344;
    let t117518 = t31868 * t32344;
    let t117527 = 55.0_f64 / 81.0_f64 * t8663 * t8513 * t8307 * t240;
    (t117496, t117499, t117516, t117518, t117527)
}
