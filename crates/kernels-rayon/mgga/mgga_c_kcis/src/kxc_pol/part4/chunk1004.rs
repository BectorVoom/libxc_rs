//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1004/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1004(t250: f64, t3106: f64, t461: f64, t453: f64, t3910: f64, t45: f64, t1314: f64, t3897: f64, t455: f64, t3900: f64, t468: f64, t11407: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11481 = t250 * t3106 * t461;
    let t11482 = 0.36793333333333333333e0_f64 * t11481;
    let t11491 = 1.0_f64/pow_3_2(t453);
    let t11500 = t45 * t3910;
    let t11512 = 1.0_f64 / t3897 / t1314;
    let t11513 = t455 * t11512;
    let t11516 = 1.0_f64 / t3900 / t468;
    let t11520 = 0.28842592592592592592e-1_f64 * t11407;
    (t11481, t11482, t11491, t11500, t11513, t11516, t11520)
}
