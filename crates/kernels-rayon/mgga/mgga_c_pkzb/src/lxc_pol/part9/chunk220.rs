//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 220/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk220(t650: f64, t677: f64, t657: f64, t668: f64, t673: f64, t681: f64) -> (f64, f64, f64) {
    let t697 = 0.516475e0_f64 * t650;
    let t700 = 0.104195e0_f64 * t677;
    let t702 = 0.3529725e1_f64 * t668 - t697 + 0.1549425e1_f64 * t657 + 0.6311625e0_f64 * t673 - t700 + 0.312585e0_f64 * t681;
    (t697, t700, t702)
}
