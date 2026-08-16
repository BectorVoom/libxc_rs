//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1003/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1003(t3753: f64, t530: f64, t174: f64, t1331: f64, t2331: f64, t3890: f64, t659: f64, t3884: f64, t251: f64, t3977: f64, t3887: f64, t11407: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11418 = 1.0_f64 / t3753 / t530;
    let t11425 = 1.0_f64 / t3753 / t174;
    let t11455 = t2331 * t1331;
    let t11457 = t659 * t3890;
    let t11460 = t659 * t3884;
    let t11462 = t251 * t3977;
    let t11475 = t659 * t3887;
    let t11479 = 0.93932222222222222223e0_f64 * t11407;
    (t11418, t11425, t11455, t11457, t11460, t11462, t11475, t11479)
}
