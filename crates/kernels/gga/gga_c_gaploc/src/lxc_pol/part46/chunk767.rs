//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 767/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk767<F: Float>(t10215: F, t1564: F, t10600: F, t1415: F, t31590: F, t493: F, t26126: F, t544: F, t18535: F, t19: F, t584: F, t60: F) -> (F, F, F, F, F) {
    let t34202 = t1564 * t10215;
    let t34264 = t1415 * t10600;
    let t34273 = t493 * t31590;
    let t34286 = t544 * t26126;
    let t34400 = t584 * t18535 * t19 * t60;
    (t34202, t34264, t34273, t34286, t34400)
}
