//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 510/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk510<F: Float>(t4439: F, t4468: F, t225: F, t1568: F, t213: F, t1580: F, t779: F, t689: F, t1579: F, t72: F, t686: F, t2465: F, t886: F, t2770: F, t1558: F, t251: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4469 = t4439 + t4468;
    let t4470 = t4469 * t225;
    let t4474 = t213 * t1568;
    let t4477 = t779 * t1580;
    let t4478 = t689 * t4477;
    let t4480 = t1579 * t72;
    let t4481 = t4480 * t686;
    let t4482 = t2465 * t4481;
    let t4486 = t1579 * t886;
    let t4487 = t2770 * t4486;
    let t4494 = t251 * t1558;
    (t4469, t4470, t4474, t4478, t4481, t4482, t4486, t4487, t4494)
}
