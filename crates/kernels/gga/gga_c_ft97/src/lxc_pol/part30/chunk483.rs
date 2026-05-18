//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 483/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk483<F: Float>(t7584: F, t7641: F, t7512: F, t7638: F, t2781: F, t1486: F, t193: F, t7611: F, t852: F, t2681: F, t27: F, t89: F) -> (F, F, F, F, F, F, F, F) {
    let t7642 = t7641 * t7584;
    let t7644 = t7638 * t7512 * t7642;
    let t7646 = t2781 * t7584;
    let t7648 = t1486 * t193 * t7646;
    let t7650 = t852 * t7611;
    let t7652 = t1486 * t193 * t7650;
    let t7654 = t2681 * t7584;
    let t7656 = t89 * t27 * t7654;
    (t7642, t7644, t7646, t7648, t7650, t7652, t7654, t7656)
}
