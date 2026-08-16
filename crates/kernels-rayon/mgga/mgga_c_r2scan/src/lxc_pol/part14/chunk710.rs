//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 710/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk710(t1923: f64, t207: f64, t1937: f64, t4741: f64, t5246: f64, t5416: f64, t5418: f64, t5422: f64, t5424: f64, t5426: f64, t1691: f64, t1986: f64) -> (f64, f64, f64) {
    let t5512 = t207 * t1923;
    let t5513 = t1937 * t5512;
    let t5523 = 0.235315e2_f64 * t5246 - 0.94126000000000000001e1_f64 * t5416 + 0.14641822222222222222e2_f64 * t5418 - 0.16831e1_f64 * t5422 + 0.11220666666666666667e1_f64 * t5424 - 0.13090777777777777778e1_f64 * t5426 - 0.32416222222222222223e0_f64 * t4741;
    let t5524 = t5523 * t207;
    let t5527 = t1986 * t1691;
    (t5513, t5524, t5527)
}
