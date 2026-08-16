//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 499/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk499(t1315: f64, t1327: f64, t1329: f64, t1333: f64, t1341: f64, t1354: f64, t1360: f64, t1363: f64, t1369: f64, t559: f64) -> f64 {
    let t1372 = -t1327 - t1315 * t1329 / 48.0_f64 + t1333 * t559 / 3072.0_f64 - t1341 * t1354 / 3072.0_f64 - t1360 - t1363 * t1369 / 768.0_f64;
    t1372
}
