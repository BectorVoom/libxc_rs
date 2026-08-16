//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 274/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk274(t1001: f64, t1003: f64, t286: f64, t285: f64, t293: f64, t984: f64, t989: f64, t991: f64, t996: f64, t296: f64) -> (f64, f64, f64, f64) {
    let t1004 = t1001 * t1003;
    let t1005 = t286 * t1004;
    let t1008 = -t984 * t293 / 36.0_f64 + t989 + t991 * t996 / 288.0_f64 - t285 * t1005 / 96.0_f64;
    let t1009 = 1.0_f64 / t296;
    (t1004, t1005, t1008, t1009)
}
