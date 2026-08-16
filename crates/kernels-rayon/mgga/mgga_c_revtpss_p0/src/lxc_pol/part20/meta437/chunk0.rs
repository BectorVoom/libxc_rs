//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1648/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1648(t12238: f64, t3428: f64, t3376: f64, t3432: f64, t3436: f64, t12358: f64, t3379: f64, t12571: f64, t3539: f64, t45021: f64, t45023: f64, t45026: f64, t45029: f64, t45033: f64, t45037: f64, t45040: f64, t45043: f64) -> (f64, f64, f64, f64, f64) {
    let t45045 = 6.0_f64 * t12238 * t3428;
    let t45046 = t3376 * t3432;
    let t45048 = 0.96491876992155210402e2_f64 * t45046 * t3436;
    let t45050 = 4.0_f64 * t3379 * t12358;
    let t45052 = 0.35089341735807877242e1_f64 * t12571 * t3539;
    let t45053 = t45021 + t45023 - t45026 - t45029 + t45033 + t45037 + t45040 + t45043 + t45045 + t45048 + t45050 - t45052;
    (t45045, t45048, t45050, t45052, t45053)
}
