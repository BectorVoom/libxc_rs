//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1324/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1324<F: Float>(t32875: F, t6547: F, t32808: F, t6562: F, t794: F, t25341: F, t30663: F, t6552: F, t112943: F, t23164: F, t7479: F, t1880: F, t25224: F, t30656: F) -> (F, F, F, F, F) {
    let t118927 = t6547 * t32875;
    let t118928 = F::cast_from(0.38381794893125283518e-1_f64) * t118927;
    let t118934 = t6562 * t794 * t32808;
    let t118935 = F::cast_from(0.82246703342411321825e-2_f64) * t118934;
    let t118938 = F::cast_from(0.3289868133696452873e-1_f64) * t6552 * t30663 * t25341;
    let t118940 = t23164 * t112943 * t7479;
    let t118941 = F::cast_from(0.16449340668482264365e-1_f64) * t118940;
    let t118944 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t25224 * t30656;
    (t118928, t118935, t118938, t118941, t118944)
}
