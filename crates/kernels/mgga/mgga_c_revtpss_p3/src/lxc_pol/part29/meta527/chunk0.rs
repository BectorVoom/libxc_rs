//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1855/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1855<F: Float>(t665: F, t94975: F, t2339: F, t624: F, t2340: F, t2366: F, t25823: F, t10208: F, t68: F, t25081: F, t7234: F, t1464: F, t7541: F) -> (F, F, F, F, F, F, F) {
    let t94976 = t94975 * t665;
    let t94978 = t624 * t2339;
    let t94979 = t94978 * t2340;
    let t94981 = t25823 * t2366;
    let t94982 = t68 * t10208;
    let t95088 = t7234 * t25081;
    let t95182 = t7541 * t1464;
    (t94976, t94978, t94979, t94981, t94982, t95088, t95182)
}
