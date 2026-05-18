//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 725/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk725<F: Float>(t182: F, t5299: F, t1647: F, t879: F, t381: F, t456: F, t5080: F, t1651: F, t955: F, t322: F, t545: F, t407: F) -> (F, F, F, F, F, F, F) {
    let t5300 = t182 * t5299;
    let t5304 = t1647 * t879;
    let t5305 = t381 * t5304;
    let t5307 = t456 * t5080;
    let t5310 = t1651 * t955;
    let t5315 = t545 * t322;
    let t5316 = t5315 * t407;
    (t5300, t5304, t5305, t5307, t5310, t5315, t5316)
}
