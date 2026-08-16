//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 919/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk919(t562: f64, t6414: f64, t172: f64, t6320: f64, t763: f64, t1819: f64, t68: f64, t1995: f64, t6330: f64, t1824: f64, t1834: f64, t6387: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19660 = t562 * t6414;
    let t19681 = t6320 * t172;
    let t19682 = t19681 * t763;
    let t19708 = t1819 * t68;
    let t19715 = t1995 * t6330;
    let t19739 = t1834 * t1824;
    let t19743 = t562 * t6387;
    (t19660, t19681, t19682, t19708, t19715, t19739, t19743)
}
