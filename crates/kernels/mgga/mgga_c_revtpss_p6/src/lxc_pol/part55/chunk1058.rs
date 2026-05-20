//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1058/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1058<F: Float>(t7536: F, t8717: F, t2014: F, t1936: F, t26399: F, t28658: F, t7002: F, t7359: F, t2055: F, t32392: F, t93: F, t7373: F, t8692: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32636 = t7536 * t8717;
    let t32637 = t2014 * t32636;
    let t32642 = F::new(2.0) * t26399 * t1936;
    let t32644 = F::new(2.0) * t28658 * t1936;
    let t32646 = F::new(2.0) * t7359 * t7002;
    let t32654 = F::new(2.0) * t32392 * t2055;
    let t32655 = t93 * t7002;
    let t32657 = F::new(2.0) * t32655 * t2055;
    let t32659 = F::new(2.0) * t8692 * t7373;
    (t32636, t32637, t32642, t32644, t32646, t32654, t32655, t32657, t32659)
}
