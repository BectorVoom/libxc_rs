//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1267/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1267(t6643: f64, t81597: f64, t23025: f64, t23030: f64, t23012: f64, t6653: f64, t22986: f64, t23175: f64, t2647: f64, t6646: f64, t10097: f64, t22641: f64, t2588: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81598 = t81597 * t6643;
    let t81599 = 0.16220877603642232915e0_f64 * t81598;
    let t81600 = t23030 * t23025;
    let t81602 = t23012 * t6653;
    let t81606 = t22986 * t6646 * t23175 * t2647;
    let t81610 = t22986 * t6646 * t10097 * t2647;
    let t81612 = t22641 * t2588;
    (t81599, t81600, t81602, t81606, t81610, t81612)
}
