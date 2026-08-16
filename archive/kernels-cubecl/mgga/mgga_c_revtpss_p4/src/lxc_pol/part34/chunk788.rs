//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 788/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk788<F: Float>(t555: F, t9646: F, t1358: F, t22: F, t1425: F, t225: F, t3907: F, t9285: F, t3906: F, t2453: F, t3914: F, t4075: F, t556: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9647 = t9646 * t555;
    let t9648 = t1358 * t22;
    let t9650 = F::cast_from(0.19637199382202157274e-3_f64) * t9647 * t9648;
    let t9655 = t1425 * t1425;
    let t9656 = F::cast_from(1.0_f64) / t9655;
    let t9657 = t225 * t9656;
    let t9664 = t3907 * t9285;
    let t9666 = F::cast_from(0.46263278077393568556e-2_f64) * t3906 * t9664;
    let t9674 = t2453 * t3914;
    let t9679 = t556 * t4075;
    (t9648, t9650, t9655, t9656, t9657, t9664, t9666, t9674, t9679)
}
