//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1153/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1153(t1967: f64, t9531: f64, t1901: f64, t7614: f64, t30468: f64, t6144: f64, t7433: f64, t9758: f64, t34481: f64, t5855: f64, t5859: f64, t8511: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39946 = t1967 * t9531;
    let t39948 = t7614 * t1901;
    let t39950 = t30468 * t6144;
    let t39952 = t7433 * t9758;
    let t39962 = t34481 * t5855;
    let t39965 = t8511 * t5859;
    (t39946, t39948, t39950, t39952, t39962, t39965)
}
