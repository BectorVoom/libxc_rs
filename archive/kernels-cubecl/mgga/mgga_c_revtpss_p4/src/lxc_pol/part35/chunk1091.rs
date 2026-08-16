//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1091/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1091<F: Float>(t233: F, t30379: F, t1957: F, t225: F, t2061: F, t5977: F, t2723: F, t25416: F, t231: F, t7076: F, t1558: F, t7997: F) -> (F, F, F, F, F, F, F, F) {
    let t30380 = t233 * t30379;
    let t30381 = t1957 * t30380;
    let t30384 = t30379 * t225;
    let t30390 = t2061 * t5977;
    let t30391 = t30390 * t2723;
    let t30392 = t25416 * t30391;
    let t30395 = t30390 * t231;
    let t30396 = t7076 * t30395;
    let t30400 = t7997 * t1558 * t231;
    (t30380, t30381, t30384, t30391, t30392, t30395, t30396, t30400)
}
