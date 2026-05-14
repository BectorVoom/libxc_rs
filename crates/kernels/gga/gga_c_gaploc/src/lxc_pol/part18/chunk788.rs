//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 788/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk788<F: Float>(t7910: F, t7948: F, t7991: F, t8038: F, t2796: F, t501: F, t1381: F, t997: F, t1016: F, t1383: F, t2902: F, t605: F, t1651: F, t2876: F, t540: F) -> (F, F, F, F, F, F, F) {
    let t8040 = t7910 + t7948 + t7991 + t8038;
    let t8042 = t2796 * t501;
    let t8045 = t997 * t1381;
    let t8054 = t1016 * t1383;
    let t8057 = t2902 * t605;
    let t8060 = t1016 * t1651;
    let t8063 = t2876 * t540;
    (t8040, t8042, t8045, t8054, t8057, t8060, t8063)
}
