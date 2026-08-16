//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 793/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk793(t2240: f64, t8301: f64, t1862: f64, t131: f64, t68: f64, t69: f64, t79: f64) -> (f64, f64, f64, f64, f64) {
    let t8302 = t2240 * t8301;
    let t8303 = t1862 * t1862;
    let t8304 = t8303 * t131;
    let t8306 = 1.0_f64 / t69 / t68;
    let t8307 = t79 * t79;
    (t8302, t8303, t8304, t8306, t8307)
}
