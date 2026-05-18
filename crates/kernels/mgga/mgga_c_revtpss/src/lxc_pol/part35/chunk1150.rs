//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1150/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1150<F: Float>(t103363: F, t25299: F, t2439: F, t780: F, t785: F, t7997: F, t25305: F, t2435: F, t28390: F, t102993: F, t25411: F, t2470: F, t28359: F) -> (F, F, F, F, F, F) {
    let t103364 = t25299 * t103363;
    let t103370 = t2439 * t785 * t7997 * t780;
    let t103394 = t25305 * t103363;
    let t103400 = t2435 * t28390;
    let t103404 = t25411 * t102993;
    let t103421 = t28359 * t2470;
    (t103364, t103370, t103394, t103400, t103404, t103421)
}
