//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 673/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk673(t3211: f64, t4805: f64, t3210: f64, t3200: f64, t1133: f64, t1773: f64) -> (f64, f64, f64) {
    let t4806 = t3211 * t4805;
    let t4807 = t3210 * t4806;
    let t4808 = t3200 * t4807;
    let t4813 = t1773 * t1133;
    (t4807, t4808, t4813)
}
