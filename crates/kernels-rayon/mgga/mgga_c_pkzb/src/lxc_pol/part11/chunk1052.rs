//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1052/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1052(t158: f64, t165: f64, t5387: f64, t1721: f64, t1511: f64, t5331: f64, t1613: f64, t4952: f64, t542: f64, t555: f64, t148: f64, t1515: f64, t1518: f64, t204: f64) -> (f64, f64, f64, f64, f64) {
    let t16421 = t158 / t5387 / t165;
    let t16425 = t1721 * t1721;
    let t16476 = t1511 * t5331;
    let t16481 = 0.46785788981077169656e1_f64 * t555 * t1613 * t4952 * t542;
    let t16486 = 0.28493333333333333333e0_f64 * t204 * t148 * t1515 * t1518;
    (t16421, t16425, t16476, t16481, t16486)
}
