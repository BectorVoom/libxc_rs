//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 887/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk887<F: Float>(t3851: F, t40897: F, t25525: F, t40901: F, t5169: F, t649: F, t36107: F, t36119: F, t41000: F, t25636: F, t2347: F, t794: F, t3839: F, t40905: F, t25518: F, t38564: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41146 = t3851 * t40897;
    let t41148 = t25525 * t40901;
    let t41150 = t649 * t5169;
    let t41151 = t36107 * t41150;
    let t41153 = t36119 * t41000;
    let t41155 = t25636 * t40901;
    let t41158 = t25525 * t2347 * t794;
    let t41160 = t3839 * t40905;
    let t41162 = t25518 * t38564;
    (t41146, t41148, t41150, t41151, t41153, t41155, t41158, t41160, t41162)
}
