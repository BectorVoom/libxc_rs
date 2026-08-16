//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1110/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1110(t22680: f64, t22946: f64, t533: f64, t1390: f64, t1983: f64, t2379: f64, t25: f64, t1914: f64, t193: f64, t201: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22947 = t22680 + t22946;
    let t22948 = t533 * t22947;
    let t22949 = t22948 * t1390;
    let t22950 = t1983 * t22949;
    let t22951 = t25 * t2379;
    let t22959 = t193 * t201 * t1914;
    (t22947, t22948, t22949, t22950, t22951, t22959)
}
