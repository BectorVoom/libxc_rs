//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 901/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk901<F: Float>(t2344: F, t39565: F, t2868: F, t7578: F, t623: F, t7191: F, t7194: F, t321: F, t8957: F, t2283: F, t35384: F, t35149: F, t39536: F, t39538: F, t39541: F, t39545: F, t39547: F, t39549: F, t39556: F, t39558: F, t39559: F, t39561: F, t39563: F, t4965: F, t739: F, t8933: F) -> (F, F) {
    let t39566 = t39565 * t2344;
    let t39568 = t2868 * t7578;
    let t39570 = t623 * t7191;
    let t39571 = t39570 * t7194;
    let t39573 = t8957 * t321;
    let t39577 = t35384 * t2283;
    let t39579 = t39536 - F::new(0.35922725105591425692e0) * t39538 + F::new(0.8980681276397856423e-1) * t39541 + t39545 - F::new(0.44903406381989282115e-1) * t39547 + F::new(0.17961362552795712846e0) * t39549 + F::new(0.79828278012425390428e-1) * t4965 * t8933 - t39556 - t39558 + F::new(0.25538759935978703638e-4) * t39559 + F::new(0.85129199786595678796e-5) * t39561 + F::new(0.27274661654245341728e-1) * t39563 + F::new(0.20455996240684006296e-1) * t39566 - F::new(0.2993560425465952141e-1) * t39568 + F::new(0.27274661654245341728e-1) * t39571 - F::new(0.11974241701863808564e0) * t739 * t39573 - F::new(0.74488049813271218947e-4) * t35149 - F::new(0.42564599893297839398e-5) * t39577;
    (t39573, t39579)
}
