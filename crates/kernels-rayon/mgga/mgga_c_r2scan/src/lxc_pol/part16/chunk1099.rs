//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1099/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1099(t37982: f64, t7606: f64, t10810: f64, t2583: f64, t574: f64, t10757: f64, t980: f64, t26176: f64, t37717: f64, t26150: f64, t37720: f64, t24573: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39785 = t37982 * t7606;
    let t39786 = 0.19514881078765566037e-1_f64 * t39785;
    let t39792 = t574 * t10810 * t2583;
    let t39793 = 0.23115257973478049502e0_f64 * t39792;
    let t39816 = t980 * t10757;
    let t39823 = t37717 * t26176;
    let t39824 = 0.47609969197673950972e-2_f64 * t39823;
    let t39825 = t37720 * t26150;
    let t39826 = 0.14282990759302185292e-1_f64 * t39825;
    let t39827 = t37717 * t24573;
    (t39786, t39793, t39816, t39824, t39826, t39827)
}
