//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1297/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1297(t32867: f64, t6547: f64, t112945: f64, t112948: f64, t118910: f64, t6552: f64, t6555: f64, t32875: f64, t32808: f64, t6562: f64, t794: f64, t25341: f64, t30663: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t118915 = t6547 * t32867;
    let t118916 = 0.38381794893125283518e-1_f64 * t118915;
    let t118917 = 0.16449340668482264365e-1_f64 * t112945;
    let t118918 = 0.82246703342411321825e-2_f64 * t112948;
    let t118924 = 0.3289868133696452873e-1_f64 * t6552 * t118910 * t6555;
    let t118927 = t6547 * t32875;
    let t118928 = 0.38381794893125283518e-1_f64 * t118927;
    let t118934 = t6562 * t794 * t32808;
    let t118935 = 0.82246703342411321825e-2_f64 * t118934;
    let t118938 = 0.3289868133696452873e-1_f64 * t6552 * t30663 * t25341;
    (t118916, t118917, t118918, t118924, t118928, t118935, t118938)
}
