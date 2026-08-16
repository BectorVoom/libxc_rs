//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1645/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1645(t16668: f64, t3385: f64, t12227: f64, t3520: f64, t5180: f64, t5206: f64, t1196: f64, t3495: f64, t1189: f64, t3543: f64, t5192: f64, t3516: f64, t5197: f64) -> (f64, f64, f64, f64, f64) {
    let t16669 = t16668 * t3385;
    let t16671 = 0.51726012919273400301e3_f64 * t12227 * t16669;
    let t16672 = t3520 * t5180;
    let t16673 = t16672 * t5206;
    let t16675 = 0.34631718211362927518e2_f64 * t1196 * t16673;
    let t16676 = t3495 * t5180;
    let t16677 = t16676 * t1189;
    let t16679 = 0.23392894490538584828e1_f64 * t1196 * t16677;
    let t16681 = 0.17315859105681463759e2_f64 * t5192 * t3543;
    let t16682 = t5197 * t3516;
    (t16671, t16675, t16679, t16681, t16682)
}
