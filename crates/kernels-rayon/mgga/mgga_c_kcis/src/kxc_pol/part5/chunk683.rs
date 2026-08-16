//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 683/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk683(t2894: f64, t4943: f64, t291: f64, t993: f64, t4581: f64, t736: f64, t992: f64) -> (f64, f64, f64, f64) {
    let t4944 = t2894 * t4943;
    let t4947 = t993 * t291;
    let t4948 = t4947 * t4581;
    let t4951 = t736 * t992;
    (t4944, t4947, t4948, t4951)
}
