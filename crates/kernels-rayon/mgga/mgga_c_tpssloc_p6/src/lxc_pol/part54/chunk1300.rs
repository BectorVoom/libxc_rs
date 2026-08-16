//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1300/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1300(t2169: f64, t7240: f64, t63: f64, t8308: f64, t113875: f64, t31860: f64, t32343: f64, t645: f64, t8513: f64, t625: f64, t79: f64, t641: f64, t8663: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t117430 = t2169 * t7240;
    let t117447 = t8308 * t63;
    let t117451 = t113875 * t63;
    let t117461 = t31860 * t8513 * t32343 * t645;
    let t117480 = t79 * t625;
    let t117483 = t8663 * t8513 * t117480 * t641;
    (t117430, t117447, t117451, t117461, t117480, t117483)
}
