//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1035/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1035<F: Float>(t33316: F, t575: F, t33338: F, t571: F, t1464: F, t8900: F, t1470: F, t644: F, t640: F, t1493: F, t36: F, t606: F, t37: F, t1497: F, t13426: F, t8460: F) -> (F, F, F, F, F, F, F, F, F) {
    let t124431 = t33316 * t575;
    let t124435 = t571 * t33338;
    let t124438 = t8900 * t1464;
    let t125260 = t1470 * t644;
    let t125268 = t1470 * t640;
    let t125279 = t1493 * t36 * t606;
    let t125312 = t37 * t606;
    let t125335 = t1497 * t36;
    let t125336 = t125335 * t606;
    let t125384 = t13426 * t8460;
    (t124431, t124435, t124438, t125260, t125268, t125279, t125312, t125336, t125384)
}
