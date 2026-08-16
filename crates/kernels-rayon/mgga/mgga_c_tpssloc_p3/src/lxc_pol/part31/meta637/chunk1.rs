//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1905/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1905(t22892: f64, t7691: f64, t90544: f64, t1835: f64, t254: f64, t28200: f64, t6883: f64, t6888: f64, t90739: f64, t1845: f64, t5187: f64, t191: f64, t192: f64, t19537: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97732 = t22892 * t90544 * t7691;
    let t97740 = t1835 * t254;
    let t97750 = t6883 * t28200;
    let t97766 = t6888 * t90739 * t7691;
    let t97789 = t5187 * t1845;
    let t97804 = t19537 * t191 * t192;
    (t97732, t97740, t97750, t97766, t97789, t97804)
}
