//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1281/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1281(t21078: f64, t3883: f64, t26: f64, t1517: f64, t21125: f64, t12: f64) -> (f64, f64) {
    let t21228 = t3883 * t21078;
    let t21229 = t26 * t21228;
    let t21233 = t1517 * t21125;
    let t21234 = t12 * t21233;
    (t21229, t21234)
}
