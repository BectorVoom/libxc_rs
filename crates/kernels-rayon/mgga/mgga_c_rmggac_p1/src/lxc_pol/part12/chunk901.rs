//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 901/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk901(t2344: f64, t39565: f64, t2868: f64, t7578: f64, t623: f64, t7191: f64, t7194: f64, t321: f64, t8957: f64, t2283: f64, t35384: f64, t35149: f64, t39536: f64, t39538: f64, t39541: f64, t39545: f64, t39547: f64, t39549: f64, t39556: f64, t39558: f64, t39559: f64, t39561: f64, t39563: f64, t4965: f64, t739: f64, t8933: f64) -> (f64, f64) {
    let t39566 = t39565 * t2344;
    let t39568 = t2868 * t7578;
    let t39570 = t623 * t7191;
    let t39571 = t39570 * t7194;
    let t39573 = t8957 * t321;
    let t39577 = t35384 * t2283;
    let t39579 = t39536 - 0.35922725105591425692e0_f64 * t39538 + 0.8980681276397856423e-1_f64 * t39541 + t39545 - 0.44903406381989282115e-1_f64 * t39547 + 0.17961362552795712846e0_f64 * t39549 + 0.79828278012425390428e-1_f64 * t4965 * t8933 - t39556 - t39558 + 0.25538759935978703638e-4_f64 * t39559 + 0.85129199786595678796e-5_f64 * t39561 + 0.27274661654245341728e-1_f64 * t39563 + 0.20455996240684006296e-1_f64 * t39566 - 0.2993560425465952141e-1_f64 * t39568 + 0.27274661654245341728e-1_f64 * t39571 - 0.11974241701863808564e0_f64 * t739 * t39573 - 0.74488049813271218947e-4_f64 * t35149 - 0.42564599893297839398e-5_f64 * t39577;
    (t39573, t39579)
}
