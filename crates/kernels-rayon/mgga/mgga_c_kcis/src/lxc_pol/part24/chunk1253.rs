//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1253/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1253(t1267: f64, t26996: f64, t5329: f64, t6837: f64, t1020: f64, t19557: f64, t7718: f64, t19561: f64, t4994: f64, t6620: f64, t92917: f64, t100129: f64, t27077: f64) -> (f64, f64, f64, f64, f64) {
    let t100466 = t5329 * t26996 * t6837 * t1267;
    let t100474 = t1020 * t7718 * t19557;
    let t100477 = t4994 * t7718 * t19561;
    let t100480 = t1020 * t92917 * t6620;
    let t100482 = t27077 * t100129;
    (t100466, t100474, t100477, t100480, t100482)
}
