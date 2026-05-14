//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 419/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk419<F: Float>(t203: F, t2293: F, t161: F, t2366: F, t123: F, t1570: F, t4260: F, t486: F, t165: F, t599: F) -> (F, F, F, F, F) {
    let t6417 = t203 * t2293;
    let t6470 = t161 * t2366;
    let t6485 = t1570 * t123;
    let t6507 = t4260 * t486;
    let t6508 = t165 * t599;
    (t6417, t6470, t6485, t6507, t6508)
}
