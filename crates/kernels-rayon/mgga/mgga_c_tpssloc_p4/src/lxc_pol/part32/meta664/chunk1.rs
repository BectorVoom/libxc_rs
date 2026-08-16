//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2096/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2096(t11147: f64, t491: f64, t1089: f64, t1751: f64, t7327: f64, t1653: f64, t7330: f64, t85822: f64, t131: f64, t1419: f64, t23598: f64, t467: f64) -> (f64, f64, f64, f64) {
    let t94797 = t491 * t11147;
    let t94837 = t7327 * t1751 * t1089;
    let t94847 = t85822 * t1653 * t7330;
    let t94858 = t1419 * t23598 * t131 * t467;
    (t94797, t94837, t94847, t94858)
}
