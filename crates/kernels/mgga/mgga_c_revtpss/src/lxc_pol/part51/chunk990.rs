//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 990/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk990<F: Float>(t125614: F, t125673: F, t125712: F, t125763: F, t125816: F, t125857: F, t125894: F, t125932: F, t1450: F, t2014: F, t532: F, t1448: F, t7933: F, t28196: F, t28197: F, t125531: F, t125532: F, t125536: F, t125537: F, t125539: F, t125541: F, t125543: F, t125545: F, t125547: F, t125550: F, t125552: F, t125554: F, t125556: F, t125558: F, t125562: F, t125566: F) -> (F,) {
    let t125938 = t2014 * t532 * (t125614 + t125673 + t125712 + t125763 + t125816 + t125857 + t125894 + t125932) * t1450;
    let t125939 = t7933 * t1448;
    let t125942 = 4.0 * t28196 * t28197 * t125939;
    let t125943 = -t125531 - 6.0 * t125532 + t125536 - 4.0 * t125537 - 4.0 * t125539 - 4.0 * t125541 - 4.0 * t125543 - 4.0 * t125545 - 4.0 * t125547 - 4.0 * t125550 - 4.0 * t125552 - 4.0 * t125554 - 4.0 * t125556 + t125558 - t125562 + t125566 + t125938 + t125942;
    (t125943,)
}
