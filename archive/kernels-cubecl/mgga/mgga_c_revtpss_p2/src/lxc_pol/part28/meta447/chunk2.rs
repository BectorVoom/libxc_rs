//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1691/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1691<F: Float>(t16750: F, t482: F, t371: F, t372: F, t1803: F, t3666: F, t1208: F, t5215: F, t225: F, t480: F, t3678: F, t5327: F) -> (F, F, F, F, F, F) {
    let t17278 = t482 * t16750;
    let t17280 = t371 * t372 * t17278;
    let t17283 = t3666 * t1803;
    let t17288 = t5215 * t1208;
    let t17289 = t17288 * t225;
    let t17290 = t17289 * t480;
    let t17296 = F::cast_from(0.28582678745379824648e-3_f64) * t5327 * t3678;
    (t17280, t17283, t17288, t17289, t17290, t17296)
}
