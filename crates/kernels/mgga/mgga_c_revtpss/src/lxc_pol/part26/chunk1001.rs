//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1001/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1001<F: Float>(t1954: F, t39643: F, t7056: F, t2453: F, t25309: F, t25304: F, t251: F, t25410: F, t2438: F, t837: F, t786: F, t92889: F, t2434: F, t25374: F, t68: F, t785: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t93139 = t1954 * t39643;
    let t93140 = t93139 * t7056;
    let t93157 = t2453 * t25309;
    let t93160 = t25304 * t25309;
    let t93169 = t2453 * t251;
    let t93170 = t93169 * t25410;
    let t93173 = t2438 * t837;
    let t93179 = t786 * t92889;
    let t93182 = t2434 * t837;
    let t93189 = t25304 * t251;
    let t93190 = t93189 * t25374;
    let t93238 = t68 * t785;
    (t93139, t93140, t93157, t93160, t93169, t93170, t93173, t93179, t93182, t93189, t93190, t93238)
}
