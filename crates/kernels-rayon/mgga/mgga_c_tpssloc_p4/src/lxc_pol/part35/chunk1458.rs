//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1458/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1458(t104589: f64, t104609: f64, t1238: f64, t2121: f64, t2123: f64, t2154: f64, t2155: f64, t21762: f64, t21776: f64, t22007: f64, t22040: f64, t22327: f64, t225: f64, t24595: f64, t27406: f64, t27751: f64, t29545: f64, t29674: f64, t29691: f64, t29795: f64, t29817: f64, t3598: f64, t45350: f64, t462: f64, t497: f64, t5055: f64, t6140: f64, t6267: f64, t7283: f64, t7285: f64, t7286: f64, t73856: f64, t8010: f64, t8087: f64) -> f64 {
    let t109888 = 0.82246703342411321825e-2_f64 * t2121 * t462 * t22327 * t225 * t497 - 3.0_f64 * t5055 * t29795 - 0.82246703342411321825e-2_f64 * t7283 * t22040 * t2123 + 0.13159472534785811492e0_f64 * t27406 * t29674 - 0.29243272299524025538e-1_f64 * t27406 * t29691 + 0.21932454224643019154e-1_f64 * t7283 * t24595 * t7286 * t21762 - 0.43864908449286038307e-1_f64 * t104589 - 0.27415567780803773942e-2_f64 * t7283 * t7285 * t7286 * t21776 + 6.0_f64 * t1238 * t3598 * t8087 * t6267 - 0.24674011002723396548e-1_f64 * t7283 * t6140 * t8010 + 0.43864908449286038307e-1_f64 * t27406 * t29817 - 0.24674011002723396548e-1_f64 * t7283 * t27751 * t29545 - 3.0_f64 * t73856 * t2155 - 0.27415567780803773942e-2_f64 * t104609 + 24.0_f64 * t1238 * t45350 * t2154 * t22007;
    t109888
}
