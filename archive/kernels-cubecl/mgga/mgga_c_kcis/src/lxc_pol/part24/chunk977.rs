//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 977/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk977<F: Float>(t18570: F, t5310: F, t15534: F, t18574: F, t1262: F, t6334: F, t3515: F, t18677: F, t18672: F, t5302: F, t1253: F, t18443: F) -> (F, F, F, F, F, F) {
    let t20632 = t5310 * t18570;
    let t20635 = t15534 * t18574;
    let t20638 = t6334 * t1262;
    let t20639 = t3515 * t20638;
    let t20642 = t5310 * t18677;
    let t20645 = t5302 * t18672;
    let t20648 = t1253 * t18443;
    (t20632, t20635, t20639, t20642, t20645, t20648)
}
