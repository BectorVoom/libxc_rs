//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 686/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk686<F: Float>(t4083: F, t688: F, t1193: F, t2417: F, t13571: F, t278: F, t10309: F, t10339: F, t10355: F, t14532: F, t14541: F, t14550: F, t14818: F, t14827: F, t14834: F, t14839: F, t14845: F, t14848: F, t2014: F, t2394: F, t2704: F, t2710: F, t274: F, t4068: F, t4069: F, t4077: F, t807: F, t8948: F, t8959: F, t8963: F, t9609: F) -> (F,) {
    let t14851 = t4083 * t688;
    let t14856 = t1193 * t2417;
    let t14859 = t13571 * t278;
    let t14868 = 0.17557713923258613e0 * t14541 * t2704 - 0.23410285231011484e0 * t14532 * t4069 + 0.33205381699090447729e-3 * t8948 * t14818 - 0.11705142615505742e0 * t4068 * t10309 + 0.23410285231011484e0 * t14550 * t274 - 0.53128610718544716366e-2 * t2014 * t14827 - 0.8854768453090786061e-3 * t8959 * t4077 - 0.8854768453090786061e-3 * t8963 * t14834 - 0.26564305359272358183e-2 * t2014 * t14839 + 0.72343824494974941953e-3 * t8963 * t14845 - 0.5116527820486904976e-1 * t10339 * t14848 + 0.639565977560863122e-1 * t2710 * t14851 - 0.25159457085530922489e-1 * t9609 * t14848 + 0.319782988780431561e-1 * t2710 * t14856 - 0.532971647967385935e-1 * t807 * t14859 + 0.27954952317256580544e-1 * t2394 * t14851 + 0.13977476158628290272e-1 * t2394 * t14856 - 0.91641760171536135284e-3 * t10355 * t14848;
    (t14868,)
}
