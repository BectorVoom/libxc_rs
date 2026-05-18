//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1073/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1073<F: Float>(t1022: F, t1865: F, t10012: F, t8669: F, t2975: F, t6081: F, t2925: F, t723: F, t1880: F, t2021: F, t8752: F, t2101: F) -> (F, F, F, F, F, F, F) {
    let t24536 = t1022 * t1865;
    let t24549 = t10012 * t8669;
    let t24554 = t6081 * t2975;
    let t24586 = t2925 * t723;
    let t24644 = t1022 * t1880;
    let t24657 = t2021 * t8752;
    let t24660 = t2101 * t2925;
    (t24536, t24549, t24554, t24586, t24644, t24657, t24660)
}
