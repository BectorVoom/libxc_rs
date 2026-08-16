//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2141/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2141<F: Float>(t92838: F, t98892: F, t27341: F, t93342: F, t93364: F, t27194: F, t689: F, t887: F, t1580: F, t2439: F, t25334: F, t2722: F, t7759: F) -> (F, F, F, F, F, F) {
    let t98897 = F::cast_from(0.51405703062096148812e-1_f64) * t92838 * t98892;
    let t98907 = F::cast_from(0.51405703062096148812e-1_f64) * t93342 * t27341;
    let t98911 = F::cast_from(0.28912093960683998208e-1_f64) * t93364 * t27341;
    let t98918 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t27194 * t887;
    let t98920 = t2439 * t25334 * t1580;
    let t98922 = t7759 * t2722;
    (t98897, t98907, t98911, t98918, t98920, t98922)
}
