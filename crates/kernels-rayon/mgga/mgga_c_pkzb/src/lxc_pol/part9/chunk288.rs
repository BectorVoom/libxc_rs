//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 288/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk288(t178: f64, t926: f64, t404: f64, t334: f64, t344: f64) -> (f64, f64, f64, f64) {
    let t927 = t178 * t926;
    let t929 = 0.14291339372689912324e-3_f64 * t404 * t927;
    let t930 = t344 * t334;
    let t931 = 1.0_f64 / t930;
    (t927, t929, t930, t931)
}
