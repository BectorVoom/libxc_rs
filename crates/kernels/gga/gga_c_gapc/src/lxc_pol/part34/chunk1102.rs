//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1102/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1102<F: Float>(t2902: F, t760: F, t103: F, t786: F, t9740: F, t2763: F, t6148: F, t2920: F, t147: F, t19: F, t2299: F, t3296: F) -> (F, F, F, F, F) {
    let t24095 = t2902 * t760;
    let t24110 = t9740 * t103 * t786;
    let t24132 = t6148 * t2763;
    let t24181 = t2920 * t760;
    let t24195 = t3296 * t2299 * t19 * t147;
    (t24095, t24110, t24132, t24181, t24195)
}
