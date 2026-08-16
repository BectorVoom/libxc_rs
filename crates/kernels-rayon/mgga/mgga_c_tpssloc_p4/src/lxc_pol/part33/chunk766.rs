//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 766/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk766(t6734: f64, t7577: f64, t1615: f64, t68: f64, t360: f64, t6744: f64) -> (f64, f64, f64) {
    let t7578 = t7577 * t6734;
    let t7581 = t1615 * t68;
    let t7582 = t7581 * t360;
    let t7583 = t6744 * t7582;
    (t7578, t7582, t7583)
}
