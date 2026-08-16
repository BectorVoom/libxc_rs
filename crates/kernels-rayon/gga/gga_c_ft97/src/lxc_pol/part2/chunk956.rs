//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 956/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk956(t14842: f64, t14844: f64, t1193: f64, t2380: f64, t4083: f64, t688: f64, t2417: f64, t13571: f64, t278: f64, t10309: f64, t10339: f64, t10355: f64, t14532: f64, t14541: f64, t14550: f64, t14818: f64, t14827: f64, t14834: f64, t14839: f64, t2014: f64, t2394: f64, t2704: f64, t2710: f64, t274: f64, t4068: f64, t4069: f64, t4077: f64, t807: f64, t8948: f64, t8959: f64, t8963: f64, t9609: f64) -> f64 {
    let t14845 = t14842 * t14844;
    let t14848 = t1193 * t2380;
    let t14851 = t4083 * t688;
    let t14856 = t1193 * t2417;
    let t14859 = t13571 * t278;
    let t14868 = 0.17557713923258613e0_f64 * t14541 * t2704 - 0.23410285231011484e0_f64 * t14532 * t4069 + 0.33205381699090447729e-3_f64 * t8948 * t14818 - 0.11705142615505742e0_f64 * t4068 * t10309 + 0.23410285231011484e0_f64 * t14550 * t274 - 0.53128610718544716366e-2_f64 * t2014 * t14827 - 0.8854768453090786061e-3_f64 * t8959 * t4077 - 0.8854768453090786061e-3_f64 * t8963 * t14834 - 0.26564305359272358183e-2_f64 * t2014 * t14839 + 0.72343824494974941953e-3_f64 * t8963 * t14845 - 0.5116527820486904976e-1_f64 * t10339 * t14848 + 0.639565977560863122e-1_f64 * t2710 * t14851 - 0.25159457085530922489e-1_f64 * t9609 * t14848 + 0.319782988780431561e-1_f64 * t2710 * t14856 - 0.532971647967385935e-1_f64 * t807 * t14859 + 0.27954952317256580544e-1_f64 * t2394 * t14851 + 0.13977476158628290272e-1_f64 * t2394 * t14856 - 0.91641760171536135284e-3_f64 * t10355 * t14848;
    t14868
}
