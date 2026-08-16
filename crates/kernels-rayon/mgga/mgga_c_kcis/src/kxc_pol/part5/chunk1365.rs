//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1365/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1365(t1546: f64, t22452: f64, t21971: f64, t555: f64, t583: f64, t578: f64, t1543: f64, t7275: f64, t1529: f64, t7314: f64, t2050: f64, t4291: f64) -> (f64, f64, f64, f64, f64) {
    let t22453 = t1546 * t22452;
    let t22455 = t555 * t21971;
    let t22456 = t583 * t22455;
    let t22457 = t578 * t22456;
    let t22459 = t1543 * t7275;
    let t22461 = t1529 * t7314;
    let t22463 = t2050 * t4291;
    (t22453, t22457, t22459, t22461, t22463)
}
