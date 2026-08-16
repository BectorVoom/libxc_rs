//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2369/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2369(t138: f64, t785: f64, t9302: f64, t2786: f64, t10073: f64, t10920: f64, t231: f64, t2760: f64, t2782: f64, t2783: f64, t836: f64, t10871: f64, t14545: f64, t39709: f64) -> (f64, f64, f64, f64, f64) {
    let t40270 = t138 * t9302 * t785;
    let t40271 = t40270 * t2786;
    let t40273 = t10073 * t10920;
    let t40278 = t2782 * t2783 * t2760 * t836 * t231;
    let t40282 = t2782 * t14545 * t39709 * t10871;
    (t40270, t40271, t40273, t40278, t40282)
}
