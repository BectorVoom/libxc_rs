//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 912/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk912<F: Float>(t1850: F, t6731: F, t6735: F, t696: F, t2477: F, t3293: F, t1060: F, t6738: F, t4684: F, t6734: F, t1814: F, t220: F, t1824: F, t6743: F, t1806: F, t6747: F) -> (F, F, F, F, F, F, F, F) {
    let t16105 = 0.93706135855523581992e-2 * t1850 * t6731;
    let t16107 = 0.93706135855523581992e-2 * t696 * t6735;
    let t16108 = t2477 * t3293;
    let t16111 = t6738 * t1060;
    let t16114 = t6734 * t4684;
    let t16117 = t1814 * t220;
    let t16118 = t16117 * t1824;
    let t16122 = 0.93706135855523581992e-2 * t696 * t6743;
    let t16124 = 0.28111840756657074598e-1 * t1806 * t6747;
    (t16105, t16107, t16108, t16111, t16114, t16118, t16122, t16124)
}
