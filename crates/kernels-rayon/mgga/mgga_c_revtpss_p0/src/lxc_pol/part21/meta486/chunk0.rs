//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2068/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2068(t11528: f64, t4595: f64, t11294: f64, t4636: f64, t4632: f64, t934: f64, t2874: f64, t1610: f64, t2918: f64, t2875: f64, t4635: f64, t11299: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15377 = 4.0_f64 * t11528 * t4595;
    let t15379 = 0.32163958997385070134e2_f64 * t11294 * t4636;
    let t15380 = t4632 * t934;
    let t15382 = 4.0_f64 * t2874 * t15380;
    let t15383 = t1610 * t2918;
    let t15385 = 2.0_f64 * t2874 * t15383;
    let t15386 = t4635 * t2875;
    let t15388 = 0.96491876992155210402e2_f64 * t11299 * t15386;
    (t15377, t15379, t15380, t15382, t15383, t15385, t15386, t15388)
}
