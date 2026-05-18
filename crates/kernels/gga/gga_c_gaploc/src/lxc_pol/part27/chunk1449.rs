//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1449/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1449<F: Float>(t29233: F, t29242: F, t32104: F, t32106: F, t32110: F, t32117: F, t32119: F, t32123: F, t32125: F, t32128: F, t32131: F, t32135: F, t32139: F, t32143: F) -> F {
    let t39362 = t29233 - t29242 - t32104 + t32106 + t32110 - t32117 - t32119 - t32123 + t32125 + t32128 - t32131 - t32135 - t32139 - t32143;
    t39362
}
