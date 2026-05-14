//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 932/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk932<F: Float>(t11237: F, t11252: F, t11263: F, t12025: F, t12026: F, t12027: F, t12028: F, t12030: F, t12031: F, t12033: F, t12034: F, t12035: F, t12036: F, t12580: F, t209: F, t3903: F, t575: F) -> (F, F, F) {
    let t12584 = -0.5431140175846100239e-5 * t11237 - t12025 + t12026 - t12027 + t12028 - 0.59742541934307102629e-4 * t11252 + t12030 + t12031 - 0.5431140175846100239e-5 * t11263 + t12033 - t12034 - t12035 + t12036;
    let t12585 = t12580 + t12584;
    let t12586 = t12585 * t209;
    let t12587 = t3903 * t575;
    (t12585, t12586, t12587)
}
