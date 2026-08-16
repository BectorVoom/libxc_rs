//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1979/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1979(t101499: f64, t16673: f64, t226: f64, t235: f64, t2617: f64, t26661: f64, t29000: f64, t29041: f64, t4234: f64, t5585: f64, t7102: f64, t808: f64, t812: f64, t81600: f64, t84851: f64, t84962: f64, t87119: f64, t87127: f64, t87140: f64, t98416: f64, t98420: f64, t98425: f64, t98428: f64, t98432: f64, t98435: f64) -> f64 {
    let t101656 = -t87119 + t808 * t29041 + 2.0_f64 * t2617 * t29000 + 2.0_f64 * t812 * t84962 * t5585 - t84851 + 0.52089578783527170489e-1_f64 * t81600 + t87127 + 0.15352717957250113407e0_f64 * t98416 - 2.0_f64 * t812 * t26661 * t4234 - t16673 * t7102 + 0.6579736267392905746e-1_f64 * t87140 + t226 * t235 * t101499 - 0.15352717957250113407e0_f64 * t98420 + 0.3289868133696452873e-1_f64 * t98425 - 0.3289868133696452873e-1_f64 * t98428 + 0.3289868133696452873e-1_f64 * t98432 - 0.16449340668482264365e-1_f64 * t98435;
    t101656
}
