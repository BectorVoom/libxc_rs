//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 751/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk751<F: Float>(t2115: F, t2170: F, t573: F, t8616: F, t8728: F, t8733: F, t8905: F, t3140: F, t3736: F, t1276: F, t1243: F, t197: F, t532: F) -> (F, F, F, F) {
    let t8909 = 3.0 * t2115 * t2170 + t573 * t8905 + t8616 + t8728 + t8733;
    let t8939 = t3140 * t3736;
    let t8944 = t3140 * t1276;
    let t8945 = t8944 * t1243;
    let t8995 = t197 * t532;
    (t8909, t8939, t8945, t8995)
}
