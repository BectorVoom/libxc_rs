//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1019/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1019(t1184: f64, t3739: f64, t852: f64, t2240: f64, t1185: f64, t9976: f64, t3033: f64, t3766: f64, t3769: f64, t8219: f64, t2242: f64, t6142: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11233 = t3739 * t1184;
    let t11234 = t11233 * t852;
    let t11236 = 6.0_f64 * t2240 * t11234;
    let t11238 = 3.0_f64 * t9976 * t1185;
    let t11240 = 3.0_f64 * t3033 * t3766;
    let t11242 = 0.48245938496077605201e2_f64 * t8219 * t3769;
    let t11243 = t11233 * t2242;
    let t11245 = 0.96491876992155210402e2_f64 * t6142 * t11243;
    (t11233, t11234, t11236, t11238, t11240, t11242, t11243, t11245)
}
