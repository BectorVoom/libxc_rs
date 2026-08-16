//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 881/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk881(t2029: f64, t238: f64, t86: f64, t1979: f64, t531: f64, t1517: f64, t833: f64, t509: f64, t5867: f64, t1153: f64, t2429: f64, t368: f64, t4213: f64, t4214: f64, t4217: f64, t5133: f64, t5966: f64, t5969: f64, t5973: f64, t5977: f64, t5981: f64) -> (f64, f64, f64, f64) {
    let t5985 = t86 * t238 * t2029;
    let t5987 = t1979 * t531;
    let t5989 = t1517 * t5987 * t833;
    let t5992 = t509 * t5867;
    let t5996 = t4213 - 0.17687407407407407407e-1_f64 * t4214 - 0.26531111111111111111e-1_f64 * t4217 - 0.17687407407407407407e-1_f64 * t5966 - 0.44218518518518518518e-1_f64 * t5133 * t5969 - 0.26531111111111111111e-1_f64 * t1153 * t5973 + 0.53062222222222222222e-1_f64 * t5133 * t5977 + 0.53062222222222222222e-1_f64 * t2429 * t5981 - 0.26531111111111111111e-1_f64 * t5985 - 0.26531111111111111111e-1_f64 * t1153 * t5989 - 0.39796666666666666666e-1_f64 * t86 * t368 * t5992;
    (t5987, t5989, t5992, t5996)
}
