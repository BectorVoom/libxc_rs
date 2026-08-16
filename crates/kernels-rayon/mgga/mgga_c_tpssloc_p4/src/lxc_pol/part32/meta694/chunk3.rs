//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2159/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2159(t1824: f64, t7722: f64, t1338: f64, t28107: f64, t1336: f64, t1352: f64, t16047: f64, t1814: f64, t1825: f64, t19654: f64, t19744: f64, t26401: f64, t26403: f64, t26453: f64, t5250: f64, t5287: f64, t5334: f64, t5344: f64, t81147: f64, t81149: f64, t81154: f64, t81187: f64, t81197: f64, t90952: f64, t97158: f64, t97161: f64, t97172: f64, t97179: f64, t97181: f64) -> (f64, f64) {
    let t97189 = t7722 * t1824;
    let t97193 = t1338 * t28107;
    let t97196 = 0.49348022005446793095e-1_f64 * t97158 - 0.24674011002723396548e-1_f64 * t97161 - t81147 - 2.0_f64 * t1336 * t90952 * t1825 + 2.0_f64 * t1814 * t26401 - 2.0_f64 * t5344 * t26403 * t5287 - 0.82246703342411321824e-2_f64 * t81149 + t81154 - 6.0_f64 * t16047 * t97172 * t19744 + 6.0_f64 * t5334 * t97172 * t5250 - 0.11514538467937585055e0_f64 * t97179 + 2.0_f64 * t5334 * t97181 * t5250 + 4.0_f64 * t19654 * t26453 - 0.12793931631041761173e0_f64 * t81187 + 0.16449340668482264365e-1_f64 * t81197 + 4.0_f64 * t5334 * t97189 * t5250 - t1336 * t97193 * t1352;
    (t97189, t97196)
}
