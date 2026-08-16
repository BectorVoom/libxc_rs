//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1939/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1939<F: Float>(t2248: F, t77: F, t7705: F, t10301: F, t1470: F, t2247: F, t4181: F, t4187: F, t10309: F, t13388: F, t76: F, t13269: F, t607: F) -> (F, F, F, F, F, F, F) {
    let t101234 = t77 * t7705 * t2248;
    let t101237 = t10301 * t1470;
    let t101240 = t2247 * t4181;
    let t101243 = t2247 * t4187;
    let t101252 = t10309 * t1470;
    let t101303 = t76 * t13388;
    let t101323 = t13269 * t607;
    (t101234, t101237, t101240, t101243, t101252, t101303, t101323)
}
