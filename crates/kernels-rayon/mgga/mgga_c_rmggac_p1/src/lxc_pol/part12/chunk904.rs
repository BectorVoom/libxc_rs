//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 904/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk904(t1971: f64, t495: f64, t5888: f64, t7230: f64, t875: f64, t3351: f64, t498: f64, t7231: f64, t2025: f64, t30221: f64, t35152: f64, t35184: f64, t35188: f64, t39584: f64, t39589: f64, t39591: f64, t39595: f64, t39600: f64, t39605: f64, t39607: f64, t39609: f64, t39615: f64, t39620: f64, t4041: f64, t8866: f64) -> f64 {
    let t39625 = t7230 * t1971 * t875 * t5888 * t495;
    let t39630 = t3351 * t7231 * t875 * t5888 * t498;
    let t39632 = 0.24829349937757072982e-4_f64 * t35152 + 0.12769379967989351819e-4_f64 * t39584 - 0.35913881159970051992e-4_f64 * t39589 - 0.74488049813271218945e-4_f64 * t39591 + 0.79828278012425390428e-1_f64 * t30221 * t2025 - 0.2993560425465952141e-1_f64 * t39595 + 0.12769379967989351819e-4_f64 * t39600 + 0.42564599893297839398e-5_f64 * t39605 - 0.42564599893297839398e-5_f64 * t39607 - 0.72042316457491791906e-3_f64 * t39609 - 0.54549323308490683458e-1_f64 * t35184 - 0.27274661654245341729e-1_f64 * t35188 + 0.11974241701863808564e0_f64 * t4041 * t8866 - 0.85129199786595678796e-5_f64 * t39615 + 0.1064114997332445985e-4_f64 * t39620 - 0.212822999466489197e-4_f64 * t39625 - 0.17025839957319135759e-4_f64 * t39630;
    t39632
}
