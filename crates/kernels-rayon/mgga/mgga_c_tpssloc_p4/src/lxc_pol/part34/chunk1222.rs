//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1222/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1222(t101694: f64, t105574: f64, t105578: f64, t105582: f64, t105586: f64, t105596: f64, t105601: f64, t1510: f64, t16673: f64, t20861: f64, t24255: f64, t26661: f64, t5612: f64, t7837: f64, t812: f64, t84851: f64, t87140: f64, t87155: f64, t98399: f64, t98416: f64, t98420: f64, t98446: f64, t98488: f64) -> f64 {
    let t108189 = -3.0_f64 * t812 * t26661 * t5612 - 3.0_f64 * t812 * t101694 * t1510 - 0.9869604401089358619e-1_f64 * t105574 - 0.9869604401089358619e-1_f64 * t105578 + 0.29608813203268075857e0_f64 * t105582 + 0.24674011002723396548e-1_f64 * t98399 + 0.9869604401089358619e-1_f64 * t105586 + 6.0_f64 * t812 * t24255 * t20861 - t84851 + 0.46058153871750340221e0_f64 * t98416 + 0.9869604401089358619e-1_f64 * t87140 - 0.46058153871750340221e0_f64 * t98420 + 0.9869604401089358619e-1_f64 * t105596 - 0.9869604401089358619e-1_f64 * t98446 - 0.49348022005446793095e-1_f64 * t105601 - 3.0_f64 * t16673 * t7837 + 0.15626873635058151147e0_f64 * t87155 + 0.11514538467937585055e0_f64 * t98488;
    t108189
}
