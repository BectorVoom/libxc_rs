//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 952/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk952<F: Float>(t1936: F, t26399: F, t28658: F, t7002: F, t7359: F, t2055: F, t32392: F, t93: F, t7373: F, t8692: F, t25805: F, t28025: F, t32176: F, t32178: F, t32389: F, t32609: F, t670: F, t6985: F, t8564: F) -> (F, F) {
    let t32642 = 2.0 * t26399 * t1936;
    let t32644 = 2.0 * t28658 * t1936;
    let t32646 = 2.0 * t7359 * t7002;
    let t32654 = 2.0 * t32392 * t2055;
    let t32655 = t93 * t7002;
    let t32657 = 2.0 * t32655 * t2055;
    let t32659 = 2.0 * t8692 * t7373;
    let t32660 = 2.0 * t2055 * t25805 + 2.0 * t2055 * t28025 + 2.0 * t32389 * t670 + 2.0 * t6985 * t7373 + t32176 + t32178 + t32609 + t32642 + t32644 + t32646 + t32654 + t32657 + t32659 + t8564;
    (t32655, t32660)
}
