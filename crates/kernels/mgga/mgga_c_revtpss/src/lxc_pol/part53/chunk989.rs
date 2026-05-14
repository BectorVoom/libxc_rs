//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 989/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk989<F: Float>(t119457: F, t1925: F, t8442: F, t32885: F, t575: F, t2037: F, t7700: F, t1455: F, t8776: F, t25081: F, t8763: F, t136: F, t8736: F, t10309: F, t2247: F, t26754: F) -> (F, F, F, F, F, F, F, F, F) {
    let t121656 = t119457 * t1925;
    let t121661 = t8442 * t1925;
    let t122806 = t32885 * t575;
    let t122809 = t2037 * t7700;
    let t122813 = t1455 * t8776;
    let t122820 = t8763 * t25081;
    let t122885 = t8736 * t136;
    let t122886 = t10309 * t122885;
    let t122890 = t2247 * t26754 * t136;
    (t121656, t121661, t122806, t122809, t122813, t122820, t122885, t122886, t122890)
}
