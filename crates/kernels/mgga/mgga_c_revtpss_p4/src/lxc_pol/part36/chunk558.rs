//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 558/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk558<F: Float>(t354: F, t471: F, t3766: F, t487: F, t460: F, t3302: F, t3603: F, t3781: F, t1811: F, t473: F, t1450: F, t1907: F) -> (F, F, F, F, F, F, F, F) {
    let t5457 = t354 * t471;
    let t5462 = t3766 * t487;
    let t5463 = t460 * t5462;
    let t5464 = t3302 * t3603;
    let t5477 = t3781 * t487;
    let t5478 = t460 * t5477;
    let t5486 = t473 * t1811;
    let t5532 = t1907 * t1450;
    (t5457, t5462, t5463, t5464, t5477, t5478, t5486, t5532)
}
