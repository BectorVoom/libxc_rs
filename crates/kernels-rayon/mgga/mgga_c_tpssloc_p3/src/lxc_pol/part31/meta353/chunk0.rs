//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1266/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1266(t248: f64, t3521: f64, t4733: f64, t1227: f64, t3536: f64, t4997: f64, t3570: f64, t5012: f64, t1213: f64, t3535: f64, t5018: f64, t1202: f64, t5023: f64) -> (f64, f64, f64, f64, f64) {
    let t15486 = t248 * t3521 * t4733;
    let t15488 = t1227 * t15486 / 3456.0_f64;
    let t15490 = t3536 * t4997 / 2304.0_f64;
    let t15492 = t248 * t3570 * t5012;
    let t15494 = t1213 * t15492 / 2304.0_f64;
    let t15495 = t3535 * t5018;
    let t15498 = t1202 * t5023;
    (t15488, t15490, t15494, t15495, t15498)
}
