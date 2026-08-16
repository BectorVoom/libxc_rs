//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1211/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1211(t82046: f64, t24255: f64, t24256: f64, t2613: f64, t2617: f64, t7104: f64, t812: f64, t81980: f64, t81987: f64, t81989: f64, t82003: f64, t82005: f64, t82011: f64, t82013: f64, t82016: f64, t82021: f64, t82025: f64, t82028: f64, t82032: f64, t82039: f64, t82043: f64, t82050: f64, t9981: f64) -> f64 {
    let t85027 = 0.55440370401180965083e0_f64 * t82046;
    let t85031 = -0.69087230807625510332e0_f64 * t81980 - 0.39478417604357434476e0_f64 * t81987 + 0.23029076935875170111e0_f64 * t81989 - 0.16449340668482264365e-1_f64 * t82003 + 0.23029076935875170111e0_f64 * t82005 - 0.38381794893125283518e0_f64 * t82011 - 0.23029076935875170111e0_f64 * t82013 - 0.49348022005446793095e-1_f64 * t82016 - 0.9869604401089358619e-1_f64 * t82021 + 0.9869604401089358619e-1_f64 * t82025 + 6.0_f64 * t812 * t24255 * t9981 + 0.24674011002723396548e-1_f64 * t82028 + 6.0_f64 * t2617 * t24256 - 0.15626873635058151147e0_f64 * t82032 - 0.31253747270116302294e0_f64 * t82039 + 0.16449340668482264365e-1_f64 * t82043 - t85027 + 0.29608813203268075857e0_f64 * t82050 + 3.0_f64 * t2613 * t7104;
    t85031
}
