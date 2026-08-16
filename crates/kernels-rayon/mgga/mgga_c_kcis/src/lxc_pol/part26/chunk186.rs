//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 186/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk186(t10: f64, t128: f64, t126: f64, t69: f64, t15: f64, t24: f64, t728: f64, t97: f64, t127: f64, t129: f64, t130: f64, t60: f64, t684: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t756 = t10 * t128;
    let t760 = t126 * t126;
    let t761 = t760 * t760;
    let t762 = t761 * t126;
    let t763 = t69 * t762;
    let t764 = t24 * t15;
    let t768 = t97 * t728;
    let t774 = 0.13140859333333333333e-2_f64 * t127 * t756 * t130 - 0.98556444999999999995e-3_f64 * t763 * t764 * t130 - 0.19711288999999999999e-2_f64 * t127 * t129 * t768 - 4.0_f64 * t60 * t684;
    (t756, t762, t763, t764, t768, t774)
}
