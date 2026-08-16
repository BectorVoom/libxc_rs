//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 724/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk724(t2308: f64, t2311: f64, t237: f64, t88: f64, t2333: f64, t2339: f64, t2341: f64, t661: f64, t2371: f64, t52: f64, t2375: f64, t8656: f64) -> (f64, f64, f64, f64) {
    let t8674 = 0.10685e0_f64 * t237 * t88 * t2308 * t2311;
    let t8678 = 0.48245472966453314466e2_f64 * t2339 * t2333 * t2341 * t661;
    let t8680 = 1.0_f64 / t2371 / t52;
    let t8682 = t8680 * t8656 * t2375;
    (t8674, t8678, t8680, t8682)
}
