//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2106/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2106(t2627: f64, t7510: f64, t13171: f64, t1510: f64, t2633: f64, t6657: f64, t812: f64, t81599: f64, t81600: f64, t81718: f64, t829: f64, t87097: f64, t87101: f64, t87104: f64, t87109: f64, t87114: f64, t87117: f64, t87119: f64, t87124: f64, t87127: f64, t87133: f64, t87135: f64, t87140: f64) -> f64 {
    let t87142 = t2627 * t7510;
    let t87146 = -0.82246703342411321825e-2_f64 * t87097 + t87101 + 0.49348022005446793095e-1_f64 * t87104 - 0.82246703342411321825e-2_f64 * t87109 + 0.9869604401089358619e-1_f64 * t87114 + 0.3289868133696452873e-1_f64 * t87117 - t87119 - t812 * t81718 * t1510 - 0.3289868133696452873e-1_f64 * t87124 - t81599 + 0.52089578783527170488e-1_f64 * t81600 + t87127 - t812 * t6657 * t13171 + 0.3289868133696452873e-1_f64 * t87133 - 2.0_f64 * t812 * t87135 * t829 + 0.16449340668482264365e-1_f64 * t87140 + 2.0_f64 * t812 * t87142 * t2633;
    t87146
}
