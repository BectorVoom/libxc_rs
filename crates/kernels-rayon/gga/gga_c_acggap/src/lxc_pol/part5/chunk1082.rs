//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1082/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1082(t1941: f64, t3952: f64, t11553: f64, t103: f64, t10952: f64, t11545: f64, t11549: f64, t11552: f64, t11557: f64, t11560: f64, t1674: f64, t1679: f64, t19289: f64, t3984: f64, t495: f64, t5399: f64, t5439: f64, t560: f64, t6583: f64, t694: f64, t811: f64, t922: f64, t96: f64) -> (f64, f64) {
    let t19387 = t1941 * t3952;
    let t19394 = 0.24415263074675393405e-3_f64 * t11553;
    let t19395 = -24.0_f64 * t103 * t10952 * t3984 * t560 * t96 - 6.0_f64 * t1674 * t6583 * t922 + 2.0_f64 * t1679 * t19387 * t811 + 6.0_f64 * t19289 * t495 * t694 - 12.0_f64 * t5399 * t5439 * t694 + t11545 + t11549 - t11552 - t11557 - t11560 + t19394;
    (t19394, t19395)
}
