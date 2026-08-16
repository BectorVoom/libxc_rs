//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 817/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk817(t552: f64, t8717: f64, t6804: f64, t3363: f64, t5093: f64, t1642: f64, t3366: f64, t7906: f64, t7907: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8718 = t8717 * t552;
    let t8719 = 0.18311447306006545054e-3_f64 * t8718;
    let t8720 = 0.48830526149350786811e-3_f64 * t6804;
    let t8721 = t5093 * t3363;
    let t8726 = t1642 * t3366;
    let t8729 = -t7906 - t7907;
    (t8718, t8719, t8720, t8721, t8726, t8729)
}
