//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1175/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1175(t23788: f64, t46252: f64, t12458: f64, t40611: f64, t2235: f64, t2244: f64, t71: f64, t9338: f64, t33: f64, t39046: f64, t608: f64, t9239: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83651 = t23788 * t46252;
    let t83695 = t40611 * t12458;
    let t83699 = t2235 * t2244;
    let t83706 = t71 * t9338;
    let t83710 = t39046 * t33;
    let t83717 = t9239 * t608;
    (t83651, t83695, t83699, t83706, t83710, t83717)
}
