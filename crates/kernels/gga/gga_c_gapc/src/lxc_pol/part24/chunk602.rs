//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 602/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk602<F: Float>(t125: F, t1458: F, t144: F, t667: F, t101: F, t1474: F, t122: F, t1572: F, t1971: F, t457: F, t521: F) -> (F, F, F, F, F, F) {
    let t3938 = t1458 * t125;
    let t3940 = t667 * t144;
    let t3945 = t1474 * t101;
    let t3946 = t1572 * t122;
    let t3948 = t1971 * t144;
    let t3949 = t521 * t457;
    (t3938, t3940, t3945, t3946, t3948, t3949)
}
