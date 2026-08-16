//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 150/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk150<F: Float>(t15: F, t580: F, t14: F, t2: F, t11: F, t22: F, t21: F, t3: F, t20: F, t12: F, t19: F, t27: F) -> (F, F, F, F, F, F, F) {
    let t582 = F::cast_from(2.0_f64) * t15 * t580;
    let t583 = t14 * t2;
    let t584 = t11 * t583;
    let t586 = F::cast_from(4.0_f64) * t584 * t22;
    let t587 = t21 * t3;
    let t588 = F::cast_from(1.0_f64) / t587;
    let t590 = F::cast_from(4.0_f64) * t20 * t588;
    let t592 = t12 * t19 * t2;
    let t594 = F::cast_from(6.0_f64) * t592 * t27;
    (t582, t583, t586, t587, t588, t590, t594)
}
