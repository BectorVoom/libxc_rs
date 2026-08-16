//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 817/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk817(t1022: f64, t6486: f64, t3227: f64, t1092: f64, t1767: f64, t1773: f64) -> (f64, f64, f64, f64) {
    let t6487 = t1022 * t6486;
    let t6488 = t3227 * t6487;
    let t6489 = t1092 * t6488;
    let t6491 = t1767 * t1773;
    (t6487, t6488, t6489, t6491)
}
