//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1233/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1233(t108311: f64, t2047: f64, t2053: f64, t20936: f64, t21033: f64, t21050: f64, t218: f64, t259: f64, t2718: f64, t7087: f64, t85129: f64, t855: f64, t98932: f64, t98941: f64, t98966: f64, t98983: f64, t98993: f64, t98995: f64) -> f64 {
    let t108430 = 0.23029076935875170111e0_f64 * t98932 - t85129 - 0.46058153871750340221e0_f64 * t98941 + t20936 * t2047 * t259 - 0.49348022005446793095e-1_f64 * t98966 + 0.24674011002723396548e-1_f64 * t98983 + t218 * t108311 * t259 + 2.0_f64 * t855 * t2718 * t2053 * t21033 - 6.0_f64 * t7087 * t21050 - 0.69087230807625510332e0_f64 * t98993 - 0.11514538467937585055e0_f64 * t98995;
    t108430
}
