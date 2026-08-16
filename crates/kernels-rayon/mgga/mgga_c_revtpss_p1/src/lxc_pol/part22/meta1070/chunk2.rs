//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3829/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3829(t22446: f64, t2435: f64, t14079: f64, t14100: f64, t22433: f64, t4071: f64, t46368: f64, t46369: f64, t46378: f64, t46385: f64, t46388: f64, t47800: f64, t47802: f64, t47805: f64, t47808: f64, t47811: f64, t47813: f64, t47816: f64, t47819: f64, t47825: f64, t47832: f64, t47834: f64) -> f64 {
    let t73623 = t2435 * t22446;
    let t73627 = t14100 * t14079;
    let t73634 = -t46368 + 0.2601984143835408805e-2_f64 * t47800 + 0.34146773541147097178e-1_f64 * t47802 - 0.34146773541147097178e-1_f64 * t46369 - 0.29268663035268940438e-1_f64 * t47805 - 0.21951497276451705328e-1_f64 * t47808 - 0.79025390195226139182e1_f64 * t4071 * t22433 + 0.21951497276451705328e-1_f64 * t47811 + 0.73171657588172351096e-2_f64 * t73623 + 0.52039682876708176102e-1_f64 * t47813 + 0.39274398764404314548e-3_f64 * t46378 - 0.39029762157531132074e-1_f64 * t73627 - 0.19514881078765566038e-1_f64 * t47816 + 0.10975748638225852664e-1_f64 * t47819 - 0.10975748638225852664e-1_f64 * t47825 - t46385 - t46388 + 0.78059524315062264152e-1_f64 * t47832 - 0.29268663035268940438e-1_f64 * t47834;
    t73634
}
