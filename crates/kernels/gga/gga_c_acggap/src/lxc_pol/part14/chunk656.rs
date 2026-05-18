//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 656/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk656<F: Float>(t159: F, t1907: F, t322: F, t381: F, t524: F, t545: F, t1539: F, t1160: F, t180: F, t1814: F, t3457: F, t3073: F) -> (F, F, F, F, F) {
    let t6454 = t159 * t1907;
    let t6455 = t6454 * t322;
    let t6456 = t381 * t6455;
    let t6461 = t545 * t524;
    let t6462 = t6461 * t1539;
    let t6463 = t1160 * t6462;
    let t6465 = t180 * t1814;
    let t6466 = t6465 * t3457;
    let t6467 = t3073 * t6466;
    (t6456, t6461, t6463, t6465, t6467)
}
