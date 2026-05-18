//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 902/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk902<F: Float>(t3172: F, t6634: F, t3610: F, t5265: F, t5293: F, t3153: F, t6628: F, t6622: F, t1263: F, t6587: F, t6624: F, t1247: F) -> (F, F, F, F, F, F) {
    let t20786 = t3172 * t6634;
    let t20787 = t3610 * t20786;
    let t20789 = t5293 * t5265;
    let t20795 = t6628 * t3153;
    let t20800 = t6622 * t3153;
    let t20809 = t1263 * t6587;
    let t20816 = t3172 * t6624;
    let t20817 = t1247 * t20816;
    (t20787, t20789, t20795, t20800, t20809, t20817)
}
