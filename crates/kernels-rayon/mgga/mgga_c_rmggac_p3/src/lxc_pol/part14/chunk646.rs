//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 646/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk646(t235: f64, t8619: f64, t2392: f64, t352: f64, t262: f64, t2350: f64, t321: f64, t7198: f64, t22: f64, t3924: f64, t333: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8620 = t235 * t8619;
    let t8621 = t2392 * t352;
    let t8622 = t262 * t8621;
    let t8623 = t8620 * t8622;
    let t8625 = t2350 * t321;
    let t8626 = t262 * t8625;
    let t8627 = t7198 * t8626;
    let t8629 = t3924 * t22;
    let t8630 = t235 * t8629;
    let t8631 = t2350 * t333;
    let t8632 = t262 * t8631;
    let t8633 = t8630 * t8632;
    (t8620, t8621, t8622, t8623, t8625, t8626, t8627, t8629, t8630, t8631, t8632, t8633)
}
