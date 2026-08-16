//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1832/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1832(t1043: f64, t7152: f64, t1089: f64, t7135: f64, t999: f64, t7145: f64, t1976: f64, t3042: f64, t988: f64, t993: f64, t378: f64, t8521: f64, t995: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25612 = t7152 * t1043;
    let t25613 = t25612 * t1089;
    let t25616 = t7135 * t999;
    let t25617 = t7145 * t25616;
    let t25620 = t1976 * t3042;
    let t25621 = t7145 * t25620;
    let t25624 = t988 * t988;
    let t25625 = t25624 * t993;
    let t25626 = t25625 * t378;
    let t25629 = t995 * t8521;
    (t25613, t25617, t25621, t25624, t25625, t25626, t25629)
}
