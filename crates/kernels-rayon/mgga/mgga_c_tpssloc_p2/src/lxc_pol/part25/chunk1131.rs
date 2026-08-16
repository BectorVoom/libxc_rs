//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1131/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1131(t22986: f64, t23175: f64, t2647: f64, t6646: f64, t10097: f64, t22641: f64, t2588: f64, t225: f64, t814: f64, t6648: f64, t23021: f64, t6547: f64) -> (f64, f64, f64, f64, f64) {
    let t81606 = t22986 * t6646 * t23175 * t2647;
    let t81610 = t22986 * t6646 * t10097 * t2647;
    let t81612 = t22641 * t2588;
    let t81613 = t225 * t814;
    let t81615 = t81612 * t81613 * t6648;
    let t81617 = t6547 * t23021;
    (t81606, t81610, t81613, t81615, t81617)
}
