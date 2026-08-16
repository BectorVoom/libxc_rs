//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 732/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk732(t14366: f64, t27: f64, t684: f64, t2145: f64, t3118: f64, t352: f64, t325: f64, t4616: f64, t235: f64, t2084: f64, t7263: f64, t876: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34805 = t27 * t14366;
    let t34806 = t684 * t34805;
    let t34807 = 0.15556658869458454171e0_f64 * t34806;
    let t34810 = t2145 * t27 * t3118 * t352;
    let t34812 = t325 * t4616;
    let t34813 = t235 * t34812;
    let t34820 = t7263 * t27 * t2084 * t876;
    (t34805, t34807, t34810, t34812, t34813, t34820)
}
