//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1921/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1921<F: Float>(t26485: F, t99463: F, t102986: F, t25387: F, t1580: F, t2439: F, t26434: F, t2453: F, t2458: F, t7998: F, t41040: F, t685: F) -> (F, F, F, F, F) {
    let t103142 = F::cast_from(0.51405703062096148812e-1_f64) * t99463 * t26485;
    let t103156 = F::cast_from(0.51405703062096148812e-1_f64) * t25387 * t102986;
    let t103158 = t2439 * t26434 * t1580;
    let t103161 = t2453 * t7998 * t2458;
    let t103181 = t685 * t41040;
    (t103142, t103156, t103158, t103161, t103181)
}
