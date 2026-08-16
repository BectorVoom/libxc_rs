//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1405/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1405(t12832: f64, t7413: f64, t1599: f64, t12651: f64, t7429: f64, t1610: f64, t6176: f64, t6177: f64, t6183: f64, t1601: f64, t18431: f64, t1600: f64) -> (f64, f64, f64, f64) {
    let t23173 = t12832 * t7413;
    let t23174 = t1599 * t23173;
    let t23176 = t12651 * t7429;
    let t23177 = t23176 * t1610;
    let t23178 = t6176 * t23177;
    let t23181 = t6177 * t6183;
    let t23182 = t6176 * t23181;
    let t23185 = t1601 * t18431;
    let t23186 = t1600 * t23185;
    (t23174, t23178, t23182, t23186)
}
