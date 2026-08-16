//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 429/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk429<F: Float>(t2052: F, t774: F, t1881: F, t531: F, t1589: F, t702: F, t1710: F, t314: F) -> (F, F, F, F) {
    let t2053 = t2052 * t774;
    let t2054 = t531 * t1881;
    let t2057 = t1589 * t702;
    let t2060 = t314 * t1710;
    (t2053, t2054, t2057, t2060)
}
