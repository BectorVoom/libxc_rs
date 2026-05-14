//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1099/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1099<F: Float>(t1949: F, t22: F, t25402: F, t93134: F, t1954: F, t39643: F, t7056: F, t2453: F, t251: F, t25410: F, t25304: F, t25374: F, t25398: F, t10982: F, t9646: F, t10985: F, t25422: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t93136 = t25402 * t1949 * t22;
    let t93138 = 0.43639970290213137151e-3 * t93134 * t93136;
    let t93139 = t1954 * t39643;
    let t93140 = t93139 * t7056;
    let t93142 = 0.51727911450665971904e-3 * t93140 * t93136;
    let t93169 = t2453 * t251;
    let t93170 = t93169 * t25410;
    let t93189 = t25304 * t251;
    let t93190 = t93189 * t25374;
    let t93194 = t2453 * t25398;
    let t93206 = 0.19637199382202157274e-3 * t9646 * t1949 * t10982;
    let t93210 = 0.46263278077393568556e-2 * t25422 * t10985;
    (t93138, t93139, t93142, t93169, t93170, t93189, t93190, t93194, t93206, t93210)
}
