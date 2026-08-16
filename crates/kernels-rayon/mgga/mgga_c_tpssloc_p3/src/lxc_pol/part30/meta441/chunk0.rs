//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1687/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1687(t2240: f64, t608: f64, t1864: f64, t645: f64, t1863: f64, t6489: f64, t9231: f64, t192: f64, t532: f64, t1982: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22549 = t2240 * t608;
    let t22550 = t1864 * t645;
    let t22551 = t1863 * t22550;
    let t22554 = t9231 * t6489;
    let t22573 = t192 * t532;
    let t22574 = t1982 * t22573;
    (t22549, t22550, t22551, t22554, t22573, t22574)
}
