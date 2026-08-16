//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1281/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1281(t1352: f64, t22633: f64, t6976: f64, t81052: f64, t1992: f64, t22897: f64, t3792: f64, t81094: f64, t40475: f64, t550: f64, t81028: f64, t12168: f64, t12178: f64, t12238: f64, t1336: f64, t2013: f64, t6987: f64, t81122: f64, t81125: f64, t81127: f64, t81132: f64, t81140: f64, t81147: f64, t81149: f64, t81154: f64, t81157: f64, t81160: f64, t81165: f64) -> f64 {
    let t81169 = t22633 * t6976 * t81052 * t1352;
    let t81173 = t1992 * t22897 * t81094 * t3792;
    let t81177 = t1992 * t6976 * t40475 * t550;
    let t81181 = t1992 * t22897 * t81028 * t3792;
    let t81183 = -0.24674011002723396548e-1_f64 * t81122 + 0.12337005501361698274e-1_f64 * t81125 + 0.11514538467937585055e0_f64 * t81127 - 0.49348022005446793095e-1_f64 * t81132 - t1336 * t6987 * t12178 - t1336 * t6987 * t12168 + t12238 * t2013 - 0.74022033008170189643e-1_f64 * t81140 - t81147 - 0.24674011002723396547e-1_f64 * t81149 + t81154 + 0.82246703342411321825e-2_f64 * t81157 - 0.23029076935875170111e0_f64 * t81160 - 0.14804406601634037928e0_f64 * t81165 + 0.49348022005446793095e-1_f64 * t81169 + 0.49348022005446793095e-1_f64 * t81173 - 0.82246703342411321825e-2_f64 * t81177 + 0.49348022005446793095e-1_f64 * t81181;
    t81183
}
