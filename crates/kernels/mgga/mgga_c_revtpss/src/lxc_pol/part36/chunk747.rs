//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 747/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk747<F: Float>(t3: F, t8240: F, t1918: F, t2170: F, t573: F, t7949: F, t7952: F, t7955: F, t2033: F, t4147: F, t587: F, t65: F, t3140: F, t3736: F, t1276: F, t1243: F) -> (F, F, F, F, F, F, F) {
    let t8241 = t3 * t8240;
    let t8245 = param_d * t8240;
    let t8249 = 3.0 * t1918 * t2170 + t573 * t8245 + t7949 + t7952 + t7955;
    let t8717 = t4147 * t2033;
    let t8779 = 1.0 / t65 / t587;
    let t8939 = t3140 * t3736;
    let t8944 = t3140 * t1276;
    let t8945 = t8944 * t1243;
    (t8241, t8245, t8249, t8717, t8779, t8939, t8945)
}
