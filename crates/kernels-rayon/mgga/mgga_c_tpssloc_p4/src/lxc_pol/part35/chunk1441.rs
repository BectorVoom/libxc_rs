//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1441/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1441(t1734: f64, t5392: f64, t7376: f64, t103830: f64, t103867: f64, t103877: f64, t1244: f64, t1246: f64, t2144: f64, t21510: f64, t22243: f64, t22348: f64, t22364: f64, t22389: f64, t24589: f64, t24812: f64, t24820: f64, t24821: f64, t24849: f64, t27406: f64, t27516: f64, t27550: f64, t27561: f64, t29735: f64, t29741: f64, t29758: f64, t29759: f64, t7373: f64, t7375: f64, t85963: f64, t86015: f64, t86022: f64, t86023: f64, t86076: f64, t86077: f64, t94837: f64) -> f64 {
    let t109307 = t5392 * t1734 * t7376;
    let t109324 = -0.24674011002723396548e-1_f64 * t24812 * t24820 * t22364 * t24821 + 0.82246703342411321825e-2_f64 * t85963 * t86022 * t22348 * t86023 + 0.43864908449286038307e-1_f64 * t103830 + 0.43864908449286038307e-1_f64 * t27406 * t29741 + 0.24674011002723396548e-1_f64 * t7373 * t7375 * t22389 * t7376 + 0.82246703342411321826e-2_f64 * t24589 * t27516 * t29758 - 0.16449340668482264365e-1_f64 * t24589 * t27550 * t27561 * t21510 + 0.10966227112321509577e-1_f64 * t86076 * t86077 * t109307 - 0.16449340668482264365e-1_f64 * t24849 * t94837 * t29735 - 0.16449340668482264365e-1_f64 * t24849 * t86015 * t109307 - 0.43864908449286038307e-1_f64 * t103867 + 0.82246703342411321826e-2_f64 * t103877 + 0.21932454224643019154e-1_f64 * t27406 * t29759 + t1244 * t2144 * t22243 * t1246;
    t109324
}
