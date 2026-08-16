//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2112/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2112<F: Float>(t28002: F, t686: F, t72: F, t25895: F, t5722: F, t94748: F, t1444: F, t5675: F, t98067: F, t27968: F, t3920: F, t1445: F, t27985: F, t689: F) -> (F, F, F, F, F, F, F) {
    let t98356 = t28002 * t72 * t686;
    let t98358 = F::cast_from(0.28912093960683998208e-1_f64) * t25895 * t98356;
    let t98360 = F::cast_from(0.19514881078765566038e-1_f64) * t94748 * t5722;
    let t98362 = t5675 * t1444;
    let t98368 = F::cast_from(0.28912093960683998208e-1_f64) * t25895 * t98067;
    let t98372 = t27968 * t3920;
    let t98376 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t27985 * t1445;
    (t98356, t98358, t98360, t98362, t98368, t98372, t98376)
}
