//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1149/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1149(t23682: f64, t23685: f64, t23873: f64, t780: f64, t2383: f64, t2391: f64, t7512: f64, t7516: f64, t7552: f64, t7557: f64, t7519: f64, t23660: f64, t23664: f64, t23667: f64, t23670: f64, t23673: f64, t23676: f64, t23679: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23926 = 0.31310740740740740741e1_f64 * t23682;
    let t23927 = 0.13490888888888888889e1_f64 * t23685;
    let t23928 = t780 * t23873;
    let t23931 = t7512 * t2383 * t2391;
    let t23933 = t7516 * t7552;
    let t23936 = t7557 * t2383 * t2391;
    let t23938 = t7519 * t7552;
    let t23940 = 0.24154e1_f64 * t23660 - 0.298026e1_f64 * t23664 + 0.66228e0_f64 * t23667 + 0.72462e1_f64 * t23670 - 0.80513333333333333332e0_f64 * t23673 - 0.20128333333333333334e1_f64 * t23676 - 0.108693e2_f64 * t23679 + t23926 + t23927 + 0.258925e1_f64 * t23928 + 0.11651625e2_f64 * t23931 - 0.51785e1_f64 * t23933 - 0.247573125e0_f64 * t23936 + 0.3300975e0_f64 * t23938;
    (t23928, t23931, t23933, t23936, t23938, t23940)
}
