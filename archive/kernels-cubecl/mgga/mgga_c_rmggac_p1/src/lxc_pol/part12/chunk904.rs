//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 904/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk904<F: Float>(t1971: F, t495: F, t5888: F, t7230: F, t875: F, t3351: F, t498: F, t7231: F, t2025: F, t30221: F, t35152: F, t35184: F, t35188: F, t39584: F, t39589: F, t39591: F, t39595: F, t39600: F, t39605: F, t39607: F, t39609: F, t39615: F, t39620: F, t4041: F, t8866: F) -> F {
    let t39625 = t7230 * t1971 * t875 * t5888 * t495;
    let t39630 = t3351 * t7231 * t875 * t5888 * t498;
    let t39632 = F::cast_from(0.24829349937757072982e-4_f64) * t35152 + F::cast_from(0.12769379967989351819e-4_f64) * t39584 - F::cast_from(0.35913881159970051992e-4_f64) * t39589 - F::cast_from(0.74488049813271218945e-4_f64) * t39591 + F::cast_from(0.79828278012425390428e-1_f64) * t30221 * t2025 - F::cast_from(0.2993560425465952141e-1_f64) * t39595 + F::cast_from(0.12769379967989351819e-4_f64) * t39600 + F::cast_from(0.42564599893297839398e-5_f64) * t39605 - F::cast_from(0.42564599893297839398e-5_f64) * t39607 - F::cast_from(0.72042316457491791906e-3_f64) * t39609 - F::cast_from(0.54549323308490683458e-1_f64) * t35184 - F::cast_from(0.27274661654245341729e-1_f64) * t35188 + F::cast_from(0.11974241701863808564e0_f64) * t4041 * t8866 - F::cast_from(0.85129199786595678796e-5_f64) * t39615 + F::cast_from(0.1064114997332445985e-4_f64) * t39620 - F::cast_from(0.212822999466489197e-4_f64) * t39625 - F::cast_from(0.17025839957319135759e-4_f64) * t39630;
    t39632
}
