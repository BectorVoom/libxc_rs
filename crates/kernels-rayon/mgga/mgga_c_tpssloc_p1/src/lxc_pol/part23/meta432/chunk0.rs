//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1269/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1269(t19033: f64, t4993: f64, t19046: f64, t5018: f64, t5023: f64, t6169: f64, t18321: f64, t5040: f64, t1009: f64, t22113: f64, t1011: f64, t1212: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t72302 = t19033 * t4993;
    let t72304 = t19046 * t5018;
    let t72307 = t6169 * t5023;
    let t72352 = t18321 * t5040;
    let t72361 = t22113 * t1009;
    let t72363 = t72361 * t1011 * t1212;
    (t72302, t72304, t72307, t72352, t72361, t72363)
}
