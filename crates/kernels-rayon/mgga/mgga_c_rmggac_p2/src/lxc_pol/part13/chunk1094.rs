//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1094/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1094(t2265: f64, t5321: f64, t41767: f64, t41789: f64, t41791: f64, t2466: f64, t26093: f64, t36610: f64, t36613: f64, t41745: f64, t41747: f64, t41751: f64, t41755: f64, t41760: f64, t41772: f64, t41774: f64, t41779: f64, t41784: f64, t4905: f64, t884: f64, t9530: f64) -> f64 {
    let t43836 = 0.4726e1_f64 * t5321 * t2265;
    let t43839 = 0.66211599834018861287e-4_f64 * t41767;
    let t43844 = 0.3193131120497015617e0_f64 * t41789;
    let t43850 = 0.3193131120497015617e0_f64 * t41791;
    let t43851 = -0.1702583995731913576e-4_f64 * t41745 + 0.212822999466489197e-4_f64 * t41747 + 0.5107751987195740728e-4_f64 * t41751 - 0.5107751987195740728e-4_f64 * t41755 - 0.212822999466489197e-4_f64 * t41760 - t43836 + 0.11918087970123395032e-3_f64 * t36610 - 0.5586603735995341421e-4_f64 * t36613 + t43839 - 0.85129199786595678799e-5_f64 * t41772 - 0.19863479950205658387e-3_f64 * t41774 - 0.10215503974391481456e-3_f64 * t41779 - 0.3405167991463827152e-4_f64 * t41784 + t43844 - 0.23948483403727617128e0_f64 * t884 * t9530 * t4905 + 0.59871208509319042821e-1_f64 * t26093 * t2466 + t43850;
    t43851
}
