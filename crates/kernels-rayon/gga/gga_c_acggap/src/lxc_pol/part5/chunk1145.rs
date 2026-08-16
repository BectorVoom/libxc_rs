//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1145/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1145(t1017: f64, t1150: f64, t12753: f64, t12755: f64, t1314: f64, t15574: f64, t15576: f64, t18129: f64, t20400: f64, t20519: f64, t20545: f64, t335: f64, t3616: f64, t367: f64, t4487: f64, t4593: f64, t5170: f64, t6308: f64, t6387: f64, t922: f64, t960: f64) -> f64 {
    let t20550 = -0.16006300097412701803e-1_f64 * t12753 + 0.34299214494455789578e-2_f64 * t20519 - 0.25724410870841842183e-2_f64 * t15574 - 0.45351183609335988442e-1_f64 * t15576 - 0.85748036236139473944e-3_f64 * t12755 + t1150 * t960 * t6308 * t922 / 8.0_f64 - t3616 * t960 * t6387 * t922 / 4.0_f64 + t1150 * t18129 * t1314 / 4.0_f64 - t3616 * t4593 * t4487 / 2.0_f64 - t367 * t4593 * t5170 / 8.0_f64 - t335 * t960 * t20400 * t1017 / 24.0_f64 + t367 * t960 * t20545 * t1017 / 4.0_f64;
    t20550
}
