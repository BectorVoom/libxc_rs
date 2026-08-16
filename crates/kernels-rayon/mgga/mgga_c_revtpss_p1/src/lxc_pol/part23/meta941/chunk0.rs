//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3091/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3091(t20645: f64, t57818: f64, t1149: f64, t12227: f64, t16668: f64, t6470: f64, t1189: f64, t1196: f64, t24407: f64, t3495: f64, t16676: f64, t6535: f64) -> (f64, f64, f64, f64) {
    let t81562 = 0.2894756309764656312e3_f64 * t57818 * t20645;
    let t81566 = 0.1551780387578202009e4_f64 * t12227 * t6470 * t16668 * t1149;
    let t81570 = 0.11696447245269292414e1_f64 * t1196 * t3495 * t24407 * t1189;
    let t81573 = 0.35089341735807877242e1_f64 * t1196 * t16676 * t6535;
    (t81562, t81566, t81570, t81573)
}
