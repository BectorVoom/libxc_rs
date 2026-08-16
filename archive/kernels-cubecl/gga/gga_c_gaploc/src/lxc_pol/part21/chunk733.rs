//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 733/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk733<F: Float>(t6907: F, t912: F, t587: F, t2488: F, t2487: F, t584: F, t6715: F) -> (F, F, F) {
    let t6908 = t912 * t6907;
    let t6909 = t587 * t6908;
    let t6911 = t2488 * t6907;
    let t6912 = t2487 * t6911;
    let t6914 = t584 * t6715;
    (t6909, t6912, t6914)
}
