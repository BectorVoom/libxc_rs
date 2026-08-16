//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 631/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk631(t4936: f64, t4943: f64, t7076: f64, t7122: f64, t8684: f64, t8687: f64, t8690: f64, t8702: f64, t8709: f64, t8715: f64, t8717: f64, t8721: f64, t8724: f64, t8727: f64) -> f64 {
    let t8763 = -0.1294625e1_f64 * t8702 + 0.258925e1_f64 * t8709 + t4936 + 0.20128333333333333334e0_f64 * t7076 - 0.20128333333333333333e0_f64 * t8684 + 0.60385e0_f64 * t8687 - 0.301925e0_f64 * t8690 + 0.82524375e-1_f64 * t8715 + 0.16504875e0_f64 * t8717 + t4943 + 0.22076e0_f64 * t7122 - 0.5519e-1_f64 * t8721 + 0.33114e0_f64 * t8724 - 0.16557e0_f64 * t8727;
    t8763
}
