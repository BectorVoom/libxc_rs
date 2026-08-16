//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 420/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk420(t2751: f64, t888: f64, t221: f64, t2423: f64, t2427: f64, t2430: f64, t2482: f64, t2486: f64, t2494: f64, t2529: f64, t2718: f64, t2720: f64, t2725: f64, t2729: f64, t874: f64, t889: f64) -> (f64, f64) {
    let t2752 = t2751 * t888;
    let t2764 = t2718 * t221 - 0.13345e0_f64 * t2720 * t889 + 0.890445125e-2_f64 * t2725 * t2729 - 0.66725e-1_f64 * t874 * t2752 + 0.66725e-1_f64 * t874 * t2729 + 0.21667074074074074073e-1_f64 * t2423 - 0.18571777777777777777e-1_f64 * t2427 + 0.18571777777777777777e-1_f64 * t2430 + 0.69644166666666666665e-2_f64 * t2482 - 0.13928833333333333333e-1_f64 * t2486 + 0.13928833333333333333e-1_f64 * t2494 - 0.69644166666666666665e-2_f64 * t2529;
    (t2752, t2764)
}
