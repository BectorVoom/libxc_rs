//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 616/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk616<F: Float>(t182: F, t5299: F, t1647: F, t879: F, t381: F, t456: F, t5080: F, t1651: F, t955: F, t322: F, t545: F, t407: F, t1160: F, t1251: F, t1411: F, t2925: F, t525: F) -> (F, F, F, F, F, F, F) {
    let t5300 = t182 * t5299;
    let t5304 = t1647 * t879;
    let t5305 = t381 * t5304;
    let t5307 = t456 * t5080;
    let t5310 = t1651 * t955;
    let t5315 = t545 * t322;
    let t5316 = t5315 * t407;
    let t5318 = 0.13170898365871023197e1 * t1160 * t5316;
    let t5319 = t1251 * t1411;
    let t5322 = t2925 * t525;
    (t5300, t5305, t5307, t5310, t5318, t5319, t5322)
}
