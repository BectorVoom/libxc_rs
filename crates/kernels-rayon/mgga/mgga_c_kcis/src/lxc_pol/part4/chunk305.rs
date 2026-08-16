//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 305/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk305(t1115: f64, t345: f64, t1100: f64, t1102: f64, t1106: f64, t1111: f64, t278: f64, t344: f64, t975: f64) -> (f64, f64) {
    let t1116 = t345 * t1115;
    let t1121 = t1100 + 0.65704296666666666667e-3_f64 * t1102 * t1106 + 0.1478346675e-2_f64 * t344 * t1111 - 0.98556445e-3_f64 * t344 * t1116 - 4.0_f64 * t278 * t975;
    (t1116, t1121)
}
