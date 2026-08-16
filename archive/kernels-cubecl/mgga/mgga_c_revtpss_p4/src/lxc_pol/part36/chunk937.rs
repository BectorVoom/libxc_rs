//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 937/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk937<F: Float>(t221: F, t4019: F, t6862: F, t10001: F, t6800: F, t72: F, t757: F, t1317: F, t6801: F, t1320: F, t749: F, t512: F) -> (F, F, F, F, F, F) {
    let t22182 = t4019 * t221 * t6862;
    let t22183 = t10001 * t22182;
    let t22185 = t6800 * t72;
    let t22186 = t22185 * t757;
    let t22188 = t1317 * t6801;
    let t22191 = t1320 * t6801;
    let t22195 = t6800 * t749;
    let t22196 = t512 * t22195;
    (t22182, t22183, t22186, t22188, t22191, t22196)
}
