//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 832/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk832<F: Float>(t140: F, t3247: F, t1011: F, t3254: F, t1015: F, t10326: F, t1012: F, t3237: F, t1014: F, t2852: F, t10356: F, t245: F, t3089: F, t3088: F, t3114: F, t3128: F, t372: F) -> (F, F, F, F, F, F, F, F) {
    let t11752 = t140 * t3247;
    let t11753 = t1011 * t11752;
    let t11755 = t140 * t3254;
    let t11756 = t1011 * t11755;
    let t11758 = t1015 * t10326;
    let t11759 = t1012 * t11758;
    let t11762 = t140 * t3237;
    let t11763 = t1011 * t11762;
    let t11765 = t1014 * t2852;
    let t11766 = t11765 * t10356;
    let t11767 = t1012 * t11766;
    let t11772 = t3089 * t245;
    let t11773 = t3088 * t11772;
    let t11774 = t3114 * t11773;
    let t11775 = t372 * t3128;
    (t11753, t11756, t11759, t11763, t11767, t11772, t11774, t11775)
}
