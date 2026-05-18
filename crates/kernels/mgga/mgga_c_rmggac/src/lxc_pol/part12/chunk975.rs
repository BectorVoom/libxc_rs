//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 975/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk975<F: Float>(t1971: F, t236: F, t5624: F, t7365: F, t36332: F, t36334: F, t36344: F, t36379: F, t36381: F, t36383: F, t4041: F, t40630: F, t40637: F, t40647: F, t40652: F, t40655: F, t40659: F, t40662: F, t40664: F, t40668: F, t4965: F, t8387: F, t8390: F) -> F {
    let t40672 = t7365 * t1971 * t236 * t5624;
    let t40674 = F::new(0.44903406381989282115e-1) * t40630 - F::new(0.4726e1) * t36332 - F::new(0.2363e1) * t36334 + F::new(0.51077519871957407277e-4) * t40637 - F::new(0.24829349937757072982e-4) * t36344 - F::new(0.59590439850616975158e-4) * t36379 - F::new(0.23948483403727617128e0) * t4041 * t8387 - F::new(0.23948483403727617128e0) * t4965 * t8390 - F::new(0.19863479950205658386e-4) * t36381 - F::new(0.19863479950205658386e-4) * t36383 - F::new(0.13637330827122670864e-1) * t40647 + F::new(0.12769379967989351819e-4) * t40652 - t40655 - F::new(0.31923449919973379548e-4) * t40659 - F::new(0.85129199786595678796e-5) * t40662 + F::new(0.17025839957319135759e-4) * t40664 + F::new(0.17025839957319135759e-4) * t40668 + F::new(0.85129199786595678796e-5) * t40672;
    t40674
}
