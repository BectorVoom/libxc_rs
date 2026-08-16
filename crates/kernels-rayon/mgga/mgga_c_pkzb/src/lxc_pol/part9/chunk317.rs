//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 317/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk317(t1054: f64, t626: f64, t1045: f64, t184: f64, t188: f64) -> (f64, f64) {
    let t1055 = t626 * t1054;
    let t1058 = 0.65854491829355115987e0_f64 * t1045 * t188 - 0.65854491829355115987e0_f64 * t184 * t1055;
    (t1055, t1058)
}
