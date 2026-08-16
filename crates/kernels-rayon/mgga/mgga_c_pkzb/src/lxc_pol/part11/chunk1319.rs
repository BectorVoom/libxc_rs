//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1319/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1319(t31511: f64, t881: f64, t890: f64, t898: f64, t10150: f64, t8170: f64, t3833: f64, t8028: f64, t3136: f64, t3840: f64, t10164: f64, t3147: f64) -> (f64, f64, f64, f64, f64) {
    let t31957 = 0.5848223622634646207e0_f64 * t898 * t881 * t31511 * t890;
    let t31960 = 0.31168546390226634765e3_f64 * t898 * t10150 * t8170;
    let t31962 = 0.35089341735807877242e1_f64 * t8028 * t3833;
    let t31965 = 0.10526802520742363173e2_f64 * t898 * t3840 * t3136;
    let t31967 = 0.10389515463408878255e3_f64 * t3147 * t10164;
    (t31957, t31960, t31962, t31965, t31967)
}
