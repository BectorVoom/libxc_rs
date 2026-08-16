//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 857/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk857(t12916: f64, t12919: f64, t12922: f64, t12927: f64, t12929: f64, t12931: f64, t12933: f64, t12935: f64, t12937: f64, t12939: f64, t12943: f64, t12946: f64, t12948: f64, t12954: f64) -> f64 {
    let t12956 = -0.28483875e1_f64 * t12916 + 0.46074375e0_f64 * t12919 - 0.33218518518518518518e0_f64 * t12922 - 0.29896666666666666667e0_f64 * t12927 - 0.39862222222222222223e0_f64 * t12929 + 0.29896666666666666667e0_f64 * t12931 + 0.19931111111111111111e0_f64 * t12933 - 0.27385555555555555556e0_f64 * t12935 + 0.16431333333333333333e0_f64 * t12937 + 0.5477111111111111111e-1_f64 * t12939 - 0.36514074074074074075e-1_f64 * t12943 - 0.82156666666666666667e-1_f64 * t12946 - 0.59793333333333333333e0_f64 * t12948 + 0.11958666666666666667e1_f64 * t12954;
    t12956
}
