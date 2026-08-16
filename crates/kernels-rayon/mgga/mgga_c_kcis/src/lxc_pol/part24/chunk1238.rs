//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1238/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1238(t1020: f64, t18443: f64, t3203: f64, t7718: f64, t26753: f64, t2842: f64, t28911: f64, t28915: f64, t1092: f64, t1121: f64, t26760: f64, t6700: f64) -> (f64, f64, f64, f64) {
    let t100198 = t1020 * t7718 * t3203 * t18443;
    let t100201 = t2842 * t26753 * t28911;
    let t100204 = t1020 * t26753 * t28915;
    let t100208 = t1092 * t26760 * t6700 * t1121;
    (t100198, t100201, t100204, t100208)
}
