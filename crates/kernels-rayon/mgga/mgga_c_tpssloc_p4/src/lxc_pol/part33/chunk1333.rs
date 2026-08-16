//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1333/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1333(t105661: f64, t105665: f64, t105669: f64, t105674: f64, t105685: f64, t1499: f64, t20857: f64, t20861: f64, t20870: f64, t23008: f64, t28407: f64, t28411: f64, t4166: f64, t6657: f64, t812: f64, t81689: f64, t81717: f64, t81991: f64, t82047: f64, t87635: f64, t87653: f64, t87666: f64, t87718: f64, t98564: f64, t98884: f64) -> f64 {
    let t105689 = 0.11514538467937585055e0_f64 * t98564 + 3.0_f64 * t1499 * t28407 + 6.0_f64 * t812 * t23008 * t20861 - t81689 - 0.38381794893125283518e0_f64 * t87635 - 0.24674011002723396547e-1_f64 * t87653 + t81717 + 0.49348022005446793095e-1_f64 * t105661 + 0.9869604401089358619e-1_f64 * t105665 + 0.49348022005446793095e-1_f64 * t105669 - 0.19190897446562641759e0_f64 * t87666 - 0.19739208802178717238e0_f64 * t105674 - 3.0_f64 * t4166 * t28411 - t812 * t6657 * t20870 - 6.0_f64 * t812 * t81991 * t20857 + 0.49348022005446793095e-1_f64 * t105685 - t82047 - 0.15626873635058151147e0_f64 * t87718 + 0.12337005501361698274e-1_f64 * t98884;
    t105689
}
