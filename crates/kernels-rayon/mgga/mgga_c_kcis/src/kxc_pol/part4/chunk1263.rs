//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1263/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1263(t16106: f64, t16107: f64, t3944: f64, t5613: f64, t5619: f64, t1315: f64, t5538: f64, t1336: f64, t3894: f64, t5541: f64, t1893: f64, t3898: f64) -> (f64, f64, f64, f64, f64) {
    let t16108 = t16106 * t16107;
    let t16111 = t3944 * t5613;
    let t16112 = t16111 * t5619;
    let t16115 = t5538 * t1315;
    let t16117 = 2.0_f64 * t16115 * t1336;
    let t16119 = 1.0_f64 * t5541 * t3894;
    let t16120 = t1893 * t3898;
    (t16108, t16112, t16117, t16119, t16120)
}
