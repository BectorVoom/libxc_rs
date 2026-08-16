//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2022/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2022<F: Float>(t2470: F, t28359: F, t7064: F, t7997: F, t822: F, t28313: F, t25387: F, t95822: F, t98892: F, t95537: F, t1957: F, t26550: F) -> (F, F, F, F, F, F, F, F) {
    let t103421 = t28359 * t2470;
    let t103422 = t7064 * t103421;
    let t103424 = t822 * t7997;
    let t103431 = t28313 * t2470;
    let t103432 = t25387 * t103431;
    let t103435 = F::cast_from(0.28912093960683998208e-1_f64) * t95822 * t98892;
    let t103437 = F::cast_from(0.51405703062096148812e-1_f64) * t95537 * t98892;
    let t103438 = t1957 * t26550;
    (t103421, t103422, t103424, t103431, t103432, t103435, t103437, t103438)
}
