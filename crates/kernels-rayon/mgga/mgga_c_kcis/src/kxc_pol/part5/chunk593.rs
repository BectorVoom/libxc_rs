//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 593/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk593(t1238: f64, t429: f64, t413: f64, t3246: f64, t1278: f64, t1281: f64, t1280: f64, t436: f64, t433: f64, t503: f64, t2820: f64, t509: f64, t86: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3643 = 1.0_f64 / t1238 / t429;
    let t3644 = t413 * t3643;
    let t3658 = 0.38691203703703703703e-3_f64 * t3246;
    let t3664 = t1278 * t1281;
    let t3668 = 1.0_f64 / t1280 / t436;
    let t3669 = t433 * t3668;
    let t3716 = t503 * t503;
    let t3717 = 1.0_f64 / t3716;
    let t3728 = t86 * t2820 * t509;
    (t3643, t3644, t3658, t3664, t3668, t3669, t3716, t3717, t3728)
}
