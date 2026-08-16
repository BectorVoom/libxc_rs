//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1075/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1075(t1818: f64, t236: f64, t3351: f64, t40168: f64, t498: f64, t10018: f64, t7255: f64, t36674: f64, t47570: f64, t47572: f64, t47577: f64, t47581: f64, t47585: f64, t47588: f64, t47594: f64, t47596: f64, t47598: f64, t47600: f64, t47602: f64, t47607: f64, t47612: f64, t47616: f64) -> f64 {
    let t47621 = t3351 * t40168 * t236 * t1818 * t498;
    let t47623 = t7255 * t10018;
    let t47625 = -0.85129199786595678796e-5_f64 * t47570 - 0.85129199786595678796e-5_f64 * t47572 - 0.12769379967989351819e-4_f64 * t47577 + 0.25538759935978703638e-4_f64 * t47581 - 0.38308139903968055457e-4_f64 * t47585 - 0.2993560425465952141e-1_f64 * t47588 - 0.15243824895787514157e-3_f64 * t36674 - 0.71827762319940103983e-4_f64 * t47594 + 0.17025839957319135759e-4_f64 * t47596 - 0.25538759935978703639e-4_f64 * t47598 + 0.25538759935978703639e-4_f64 * t47600 + 0.85129199786595678796e-5_f64 * t47602 - 0.85129199786595678796e-5_f64 * t47607 + 0.71827762319940103983e-4_f64 * t47612 + 0.90915538847484472429e-2_f64 * t47616 - 0.25538759935978703639e-4_f64 * t47621 - 0.42564599893297839398e-5_f64 * t47623;
    t47625
}
