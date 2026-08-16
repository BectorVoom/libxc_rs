//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 522/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk522(t1100: f64, t1661: f64, t1107: f64, t1113: f64, t1653: f64, t136: f64, t1105: f64, t1112: f64, t1655: f64, t1118: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1662 = t1100 * t1661;
    let t1665 = t1107 * t1661;
    let t1667 = t1113 * t1653;
    let t1668 = t136 * t1667;
    let t1670 = 0.1898925e1_f64 * t1662 - t1105 + 0.29896666666666666667e0_f64 * t1655 + 0.3071625e0_f64 * t1665 - t1112 + 0.82156666666666666667e-1_f64 * t1668;
    let t1671 = t1670 * t1118;
    (t1662, t1665, t1667, t1668, t1670, t1671)
}
