//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 610/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk610(t1360: f64, t1363: f64, t110: f64, t499: f64, t493: f64, t1369: f64, t24: f64) -> (f64, f64, f64, f64) {
    let t3964 = t1360 * t1363;
    let t3967 = t110 * t499;
    let t3969 = t493 * t3967 / 432.0_f64;
    let t3970 = t24 * t1369;
    (t3964, t3967, t3969, t3970)
}
