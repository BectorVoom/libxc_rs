//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 975/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk975(t1971: f64, t236: f64, t5624: f64, t7365: f64, t36332: f64, t36334: f64, t36344: f64, t36379: f64, t36381: f64, t36383: f64, t4041: f64, t40630: f64, t40637: f64, t40647: f64, t40652: f64, t40655: f64, t40659: f64, t40662: f64, t40664: f64, t40668: f64, t4965: f64, t8387: f64, t8390: f64) -> f64 {
    let t40672 = t7365 * t1971 * t236 * t5624;
    let t40674 = 0.44903406381989282115e-1_f64 * t40630 - 0.4726e1_f64 * t36332 - 0.2363e1_f64 * t36334 + 0.51077519871957407277e-4_f64 * t40637 - 0.24829349937757072982e-4_f64 * t36344 - 0.59590439850616975158e-4_f64 * t36379 - 0.23948483403727617128e0_f64 * t4041 * t8387 - 0.23948483403727617128e0_f64 * t4965 * t8390 - 0.19863479950205658386e-4_f64 * t36381 - 0.19863479950205658386e-4_f64 * t36383 - 0.13637330827122670864e-1_f64 * t40647 + 0.12769379967989351819e-4_f64 * t40652 - t40655 - 0.31923449919973379548e-4_f64 * t40659 - 0.85129199786595678796e-5_f64 * t40662 + 0.17025839957319135759e-4_f64 * t40664 + 0.17025839957319135759e-4_f64 * t40668 + 0.85129199786595678796e-5_f64 * t40672;
    t40674
}
