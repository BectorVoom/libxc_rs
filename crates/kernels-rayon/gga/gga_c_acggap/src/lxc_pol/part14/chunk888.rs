//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 888/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk888(t7538: f64, t7720: f64, t7724: f64, t30228: f64, t601: f64, t30174: f64, t151: f64, t56: f64, t593: f64, t606: f64, t30225: f64, t425: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30655 = t7538 * t7720;
    let t30657 = t7538 * t7724;
    let t30658 = 0.32155513588552302729e-3_f64 * t30657;
    let t30663 = t30228 * t601;
    let t30664 = 0.19293308153131381638e-2_f64 * t30663;
    let t30665 = 1.0_f64 / t30174;
    let t30668 = t151 * t593 * t30665 * t56;
    let t30669 = t30668 * t601;
    let t30670 = 0.36014175219178579057e-1_f64 * t30669;
    let t30671 = t30668 * t606;
    let t30672 = 0.52832795046534975474e-1_f64 * t30671;
    let t30673 = t30225 * t425;
    (t30655, t30658, t30664, t30670, t30672, t30673)
}
