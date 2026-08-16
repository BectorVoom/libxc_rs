//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1235/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1235(t40806: f64, t1010: f64, t37040: f64, t11882: f64, t19146: f64, t37043: f64, t37048: f64, t37055: f64, t37069: f64, t40779: f64, t40782: f64, t40786: f64, t40788: f64, t40790: f64, t40792: f64, t40794: f64, t40798: f64, t40800: f64, t40802: f64, t40805: f64, param_eta: f64) -> f64 {
    let t40807 = 4.0_f64 / 3.0_f64 * t40806;
    let t40808 = t37040 * t1010;
    let t40812 = t19146 * param_eta * t11882;
    let t40814 = -11.0_f64 / 9.0_f64 * t40779 + t40782 - 4.0_f64 / 3.0_f64 * t37048 + 2.0_f64 * t37055 - 2.0_f64 / 3.0_f64 * t37069 + t40786 + 22.0_f64 / 9.0_f64 * t40788 + t40790 / 4.0_f64 + t40792 / 4.0_f64 + t40794 / 2.0_f64 - t40798 + t40800 / 4.0_f64 - 3.0_f64 / 4.0_f64 * t40802 - t40805 - t40807 + 11.0_f64 / 9.0_f64 * t40808 - t37043 / 3.0_f64 - 3.0_f64 / 2.0_f64 * t40812;
    t40814
}
