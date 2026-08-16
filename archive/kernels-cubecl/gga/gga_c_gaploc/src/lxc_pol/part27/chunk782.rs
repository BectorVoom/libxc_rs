//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 782/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk782<F: Float>(t7383: F, t969: F, t825: F, t2685: F, t2684: F, t2021: F, t2032: F) -> (F, F, F) {
    let t7384 = t969 * t7383;
    let t7385 = t825 * t7384;
    let t7387 = t2685 * t7383;
    let t7388 = t2684 * t7387;
    let t7390 = t2021 * t2032;
    (t7385, t7388, t7390)
}
