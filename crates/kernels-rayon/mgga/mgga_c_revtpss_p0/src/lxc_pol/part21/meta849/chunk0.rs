//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3189/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3189(t12571: f64, t5202: f64, t1196: f64, t16676: f64, t3516: f64, t12564: f64, t5192: f64, t17164: f64, t3531: f64, t1179: f64, t1188: f64, t58456: f64) -> (f64, f64, f64, f64, f64) {
    let t58715 = 0.17544670867903938621e1_f64 * t12571 * t5202;
    let t58718 = 0.35089341735807877242e1_f64 * t1196 * t16676 * t3516;
    let t58720 = 0.5848223622634646207e0_f64 * t5192 * t12564;
    let t58722 = 0.17544670867903938621e1_f64 * t3531 * t17164;
    let t58726 = 0.5848223622634646207e0_f64 * t1196 * t1179 * t58456 * t1188;
    (t58715, t58718, t58720, t58722, t58726)
}
