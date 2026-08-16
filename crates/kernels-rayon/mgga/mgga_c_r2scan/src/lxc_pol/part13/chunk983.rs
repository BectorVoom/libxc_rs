//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 983/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk983(t1044: f64, t3424: f64, t3685: f64, t885: f64, t4176: f64, t986: f64, t3270: f64, t3269: f64, t1108: f64, t2449: f64, t1065: f64, t983: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11537 = t3424 * t1044;
    let t11538 = t3685 * t885;
    let t11539 = t4176 * t986;
    let t11540 = t3270 * t11539;
    let t11541 = t3269 * t11540;
    let t11542 = t11541 / 4.0_f64;
    let t11543 = t2449 * t1108;
    let t11544 = t1065 * t983;
    (t11537, t11538, t11540, t11541, t11542, t11543, t11544)
}
