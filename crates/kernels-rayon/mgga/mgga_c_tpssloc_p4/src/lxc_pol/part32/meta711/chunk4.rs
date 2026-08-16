//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2229/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2229(t1888: f64, t232: f64, t58262: f64, t6646: f64, t23110: f64, t23185: f64, t28422: f64, t16817: f64, t82018: f64, t16825: f64, t22996: f64, t1510: f64, t16673: f64, t16753: f64, t2617: f64, t28351: f64, t28409: f64, t28411: f64, t6657: f64, t6658: f64, t812: f64, t87101: f64, t87135: f64, t92497: f64, t98374: f64, t98380: f64, t98384: f64, t98387: f64, t98392: f64) -> f64 {
    let t98396 = t1888 * t6646 * t58262 * t232;
    let t98399 = t23185 * t23110 * t28422;
    let t98402 = t1888 * t82018 * t16817;
    let t98405 = t1888 * t22996 * t16825;
    let t98409 = -2.0_f64 * t812 * t87135 * t1510 - 0.19190897446562641759e-1_f64 * t98374 + t92497 - 2.0_f64 * t2617 * t28351 - t812 * t6657 * t16753 + 0.19190897446562641759e-1_f64 * t98380 - t2617 * t28411 - 0.16449340668482264365e-1_f64 * t98384 - 0.82246703342411321825e-2_f64 * t98387 + 0.9869604401089358619e-1_f64 * t98392 - 0.82246703342411321825e-2_f64 * t98396 + 0.41123351671205660912e-2_f64 * t98399 - 0.49348022005446793095e-1_f64 * t98402 + 0.49348022005446793095e-1_f64 * t98405 + t87101 - t16673 * t6658 - t2617 * t28409;
    t98409
}
