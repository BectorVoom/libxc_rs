//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1197/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1197(t26464: f64, t2726: f64, t8764: f64, t882: f64, t26463: f64, t213: f64, t2751: f64, t6: f64, t887: f64, t26470: f64, t26465: f64, t2746: f64, t8525: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t91966 = t26464 * t8764 * t2726 * t882;
    let t91967 = t26463 * t91966;
    let t91972 = t6 * t213 * t8764 * t887 * t2751;
    let t91973 = t26463 * t91972;
    let t91975 = t26470 * t91966;
    let t91978 = t26464 * t26465 * t2751;
    let t91979 = t26463 * t91978;
    let t91982 = t26464 * t8525 * t2746;
    (t91966, t91967, t91972, t91973, t91975, t91978, t91979, t91982)
}
