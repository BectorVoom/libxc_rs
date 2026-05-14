//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 784/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk784<F: Float>(t11460: F, t11495: F, t1809: F, t23068: F, t23070: F, t23072: F, t23074: F, t23118: F, t2505: F, t28532: F, t28539: F, t28572: F, t28575: F, t674: F, t702: F, t8616: F) -> (F,) {
    let t28578 = -3.0 * t8616 * t2505 - t28532 * t702 - 0.14055920378328537299e-1 * t23070 + 0.70279601891642686494e-2 * t23072 - 0.42167761134985611897e-1 * t23074 + 0.14055920378328537299e-1 * t23068 - 0.28111840756657074597e-1 * t23118 - t11460 - 0.23426533963880895498e-2 * t1809 * t28539 - 0.46853067927761790996e-2 * t674 * t28572 - 0.14055920378328537299e-1 * t11495 * t28575;
    (t28578,)
}
