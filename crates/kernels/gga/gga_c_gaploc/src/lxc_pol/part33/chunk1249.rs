//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1249/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1249<F: Float>(t32856: F, t32860: F, t32866: F, t32870: F, t32872: F, t32875: F, t32878: F, t32881: F, t32884: F, t32886: F, t32888: F, t32892: F, t32896: F, t32900: F, t32902: F, t32904: F) -> (F,) {
    let t38954 = t32856 + t32860 - t32866 - t32870 + t32872 - t32875 - t32878 - t32881 + t32884 - t32886 + t32888 + t32892 + t32896 + t32900 + t32902 + t32904;
    (t38954,)
}
