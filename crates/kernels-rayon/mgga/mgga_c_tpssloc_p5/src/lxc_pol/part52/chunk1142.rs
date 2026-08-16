//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1142/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1142(t27860: f64, t27867: f64, t27878: f64, t27905: f64, t3: f64, t112: f64, t8110: f64, t1458: f64, t24969: f64, t24972: f64, t26533: f64, t26535: f64, t26537: f64, t26539: f64, t26541: f64, t26544: f64, t26547: f64, t26549: f64, t26552: f64, t26554: f64, t4072: f64, t5376: f64, t577: f64, t671: f64, t7423: f64) -> (f64, f64, f64, f64) {
    let t27907 = t27860 + t27867 + t27878 + t27905;
    let t27908 = t3 * t27907;
    let t27921 = t8110 * t112;
    let t27930 = 0.45e1_f64 * t27907 * t577 + 0.135e2_f64 * t27921 * t671 + 0.135e2_f64 * t24969 * t1458 + 27.0_f64 * t24972 * t5376 + 0.135e2_f64 * t7423 * t4072 + t26533 + t26535 + t26537 + t26539 + t26541 + t26544 + t26547 + t26549 + t26552 + t26554;
    (t27907, t27908, t27921, t27930)
}
