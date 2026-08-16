//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1180/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1180(t19790: f64, t3227: f64, t1092: f64, t2825: f64, t6504: f64, t1020: f64, t2822: f64, t6630: f64, t6625: f64, t9438: f64, t3200: f64, t13155: f64, t19396: f64) -> (f64, f64, f64, f64, f64) {
    let t19791 = t3227 * t19790;
    let t19792 = t1092 * t19791;
    let t19799 = t2825 * t6504;
    let t19800 = t1020 * t19799;
    let t19802 = t2822 * t6630;
    let t19804 = t9438 * t6625;
    let t19805 = t3200 * t19804;
    let t19807 = t13155 * t19396;
    (t19792, t19800, t19802, t19805, t19807)
}
