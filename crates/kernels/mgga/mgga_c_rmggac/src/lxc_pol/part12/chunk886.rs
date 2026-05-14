//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 886/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk886<F: Float>(t6444: F, t9000: F, t25529: F, t27: F, t5178: F, t649: F, t30526: F, t8645: F, t3851: F, t39059: F, t38745: F, t39879: F, t3839: F, t39875: F, t3814: F, t39684: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41128 = t6444 * t9000;
    let t41129 = 0.15965655602485078085e0 * t41128;
    let t41130 = t25529 * t27;
    let t41132 = t41130 * t649 * t5178;
    let t41134 = t30526 * t8645;
    let t41136 = t3851 * t39059;
    let t41138 = t3851 * t38745;
    let t41140 = t3851 * t39879;
    let t41142 = t3839 * t39875;
    let t41144 = t3814 * t39684;
    (t41129, t41130, t41132, t41134, t41136, t41138, t41140, t41142, t41144)
}
