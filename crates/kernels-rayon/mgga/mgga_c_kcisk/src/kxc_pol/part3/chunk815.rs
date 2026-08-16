//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 815/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk815(t12552: f64, t841: f64, t848: f64, t2883: f64, t813: f64, t14: f64, t2886: f64, t31: f64, t12514: f64, t2917: f64, t52: f64, t12535: f64, t2921: f64) -> (f64, f64, f64, f64) {
    let t12554 = t841 * t12552 * t848;
    let t12558 = 1.0_f64 / t2883 / t813;
    let t12559 = t14 * t12558;
    let t12561 = 1.0_f64 / t2886 / t31;
    let t12562 = t12514 * t12561;
    let t12564 = 0.51725014705706168417e3_f64 * t12559 * t12562;
    let t12566 = 1.0_f64 / t2917 / t52;
    let t12568 = t12566 * t12535 * t2921;
    (t12554, t12564, t12566, t12568)
}
