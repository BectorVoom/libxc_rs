//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1181/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1181<F: Float>(t11869: F, t29861: F, t11522: F, t18551: F, t30288: F, t11924: F, t29582: F, t11910: F, t30325: F, t11917: F, t29481: F, t3402: F) -> (F, F, F, F, F) {
    let t33865 = t11869 * t29861;
    let t33868 = t18551 * t11522 * t30288;
    let t33870 = t11924 * t29582;
    let t33872 = t11910 * t30325;
    let t33875 = t3402 * t11917 * t29481;
    (t33865, t33868, t33870, t33872, t33875)
}
