//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 877/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk877<F: Float>(t1264: F, t2131: F, t2133: F, t2147: F, t3645: F, t611: F, t7908: F, t7990: F, t694: F, t7278: F, t839: F, t10409: F, t1679: F, t467: F, t104: F, t9081: F) -> (F, F, F, F, F, F) {
    let t32219 = t2131 * t2147 * t2133 * t1264;
    let t32222 = 0.65854491829355115987e0 * t3645 * t611;
    let t32223 = t7990 * t7908;
    let t32246 = t694 * t7278 * t839;
    let t32249 = t1679 * t10409 * t467;
    let t33352 = t104 * t9081;
    (t32219, t32222, t32223, t32246, t32249, t33352)
}
