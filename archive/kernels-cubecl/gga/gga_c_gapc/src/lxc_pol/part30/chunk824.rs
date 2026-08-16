//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 824/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk824<F: Float>(t3288: F, t7708: F, t9826: F, t2778: F, t9760: F, t325: F, t8998: F, t129: F, t8117: F, t3337: F, t8769: F, t916: F) -> (F, F, F, F, F, F, F) {
    let t9827 = t3288 * t7708;
    let t9828 = t9826 * t9827;
    let t9830 = t9760 * t2778;
    let t9832 = t325 * t8998;
    let t9833 = t9832 * t2778;
    let t9835 = t8117 * t129;
    let t9836 = t9835 * t3337;
    let t9838 = t916 * t8769;
    (t9827, t9828, t9830, t9832, t9833, t9836, t9838)
}
