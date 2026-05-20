//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3829/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3829<F: Float>(t22446: F, t2435: F, t14079: F, t14100: F, t22433: F, t4071: F, t46368: F, t46369: F, t46378: F, t46385: F, t46388: F, t47800: F, t47802: F, t47805: F, t47808: F, t47811: F, t47813: F, t47816: F, t47819: F, t47825: F, t47832: F, t47834: F) -> F {
    let t73623 = t2435 * t22446;
    let t73627 = t14100 * t14079;
    let t73634 = -t46368 + F::cast_from(0.2601984143835408805e-2_f64) * t47800 + F::cast_from(0.34146773541147097178e-1_f64) * t47802 - F::cast_from(0.34146773541147097178e-1_f64) * t46369 - F::cast_from(0.29268663035268940438e-1_f64) * t47805 - F::cast_from(0.21951497276451705328e-1_f64) * t47808 - F::cast_from(0.79025390195226139182e1_f64) * t4071 * t22433 + F::cast_from(0.21951497276451705328e-1_f64) * t47811 + F::cast_from(0.73171657588172351096e-2_f64) * t73623 + F::cast_from(0.52039682876708176102e-1_f64) * t47813 + F::cast_from(0.39274398764404314548e-3_f64) * t46378 - F::cast_from(0.39029762157531132074e-1_f64) * t73627 - F::cast_from(0.19514881078765566038e-1_f64) * t47816 + F::cast_from(0.10975748638225852664e-1_f64) * t47819 - F::cast_from(0.10975748638225852664e-1_f64) * t47825 - t46385 - t46388 + F::cast_from(0.78059524315062264152e-1_f64) * t47832 - F::cast_from(0.29268663035268940438e-1_f64) * t47834;
    t73634
}
