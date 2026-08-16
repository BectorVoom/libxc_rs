//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1798/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1798(t19790: f64, t19803: f64, t225: f64, t1814: f64, t5343: f64, t3901: f64, t6420: f64, t6378: f64, t68: f64) -> (f64, f64, f64, f64, f64) {
    let t19804 = t19790 + t19803;
    let t19805 = t19804 * t225;
    let t19810 = t1814 * t5343;
    let t19813 = t3901 * t6420;
    let t19815 = t6378 * t68;
    (t19804, t19805, t19810, t19813, t19815)
}
