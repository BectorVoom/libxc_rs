//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1094/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1094<F: Float>(t2265: F, t5321: F, t41767: F, t41789: F, t41791: F, t2466: F, t26093: F, t36610: F, t36613: F, t41745: F, t41747: F, t41751: F, t41755: F, t41760: F, t41772: F, t41774: F, t41779: F, t41784: F, t4905: F, t884: F, t9530: F) -> F {
    let t43836 = F::new(0.4726e1) * t5321 * t2265;
    let t43839 = F::new(0.66211599834018861287e-4) * t41767;
    let t43844 = F::new(0.3193131120497015617e0) * t41789;
    let t43850 = F::new(0.3193131120497015617e0) * t41791;
    let t43851 = -F::new(0.1702583995731913576e-4) * t41745 + F::new(0.212822999466489197e-4) * t41747 + F::new(0.5107751987195740728e-4) * t41751 - F::new(0.5107751987195740728e-4) * t41755 - F::new(0.212822999466489197e-4) * t41760 - t43836 + F::new(0.11918087970123395032e-3) * t36610 - F::new(0.5586603735995341421e-4) * t36613 + t43839 - F::new(0.85129199786595678799e-5) * t41772 - F::new(0.19863479950205658387e-3) * t41774 - F::new(0.10215503974391481456e-3) * t41779 - F::new(0.3405167991463827152e-4) * t41784 + t43844 - F::new(0.23948483403727617128e0) * t884 * t9530 * t4905 + F::new(0.59871208509319042821e-1) * t26093 * t2466 + t43850;
    t43851
}
