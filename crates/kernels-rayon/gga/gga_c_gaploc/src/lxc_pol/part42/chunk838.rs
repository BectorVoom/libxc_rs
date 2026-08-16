//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 838/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk838(t13552: f64, t731: f64, t13503: f64, t2549: f64, t40612: f64, t40614: f64, t40620: f64, t40622: f64, t40627: f64, t40630: f64, t40632: f64, t40634: f64, t471: f64) -> (f64, f64, f64, f64) {
    let t44827 = t731 * t13552;
    let t44828 = 0.42725145723012357132e-3_f64 * t44827;
    let t44829 = t731 * t13503;
    let t44837 = t2549 * t13503;
    let t44855 = (21.0_f64 / 256.0_f64 * t40612 + 357.0_f64 / 8192.0_f64 * t40614 - 189.0_f64 / 131072.0_f64 * t40620 + 189.0_f64 / 8388608.0_f64 * t40622 - 63.0_f64 / 8388608.0_f64 * t40627 + 63.0_f64 / 131072.0_f64 * t40630 - 119.0_f64 / 8192.0_f64 * t40632 - 7.0_f64 / 256.0_f64 * t40634) * t471;
    (t44828, t44829, t44837, t44855)
}
