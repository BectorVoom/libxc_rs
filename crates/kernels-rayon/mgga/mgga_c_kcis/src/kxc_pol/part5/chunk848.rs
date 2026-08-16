//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 848/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk848(t1495: f64, t6917: f64, t1468: f64, t1464: f64, t2011: f64) -> (f64, f64, f64, f64) {
    let t6918 = t1495 * t6917;
    let t6919 = t1468 * t6918;
    let t6920 = t1464 * t6919;
    let t6922 = t2011 * t2011;
    (t6918, t6919, t6920, t6922)
}
