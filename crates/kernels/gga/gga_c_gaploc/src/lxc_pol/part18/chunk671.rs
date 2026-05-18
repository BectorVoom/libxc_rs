//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 671/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk671<F: Float>(t161: F, t1890: F, t2052: F, t796: F, t2154: F, t2109: F, t806: F) -> (F, F, F, F) {
    let t5841 = t1890 * t161;
    let t5983 = t2052 * t796;
    let t6018 = t2154 * t796;
    let t6021 = t2109 * t806;
    (t5841, t5983, t6018, t6021)
}
