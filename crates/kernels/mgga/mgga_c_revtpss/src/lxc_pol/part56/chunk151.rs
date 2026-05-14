//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 151/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk151<F: Float>(t15: F, t580: F, t14: F, t2: F, t11: F, t22: F, t21: F, t3: F, t20: F, t12: F, t19: F, t27: F, t579: F) -> (F, F, F, F, F, F, F, F, F) {
    let t582 = 2.0 * t15 * t580;
    let t583 = t14 * t2;
    let t584 = t11 * t583;
    let t586 = 4.0 * t584 * t22;
    let t587 = t21 * t3;
    let t588 = 1.0 / t587;
    let t590 = 4.0 * t20 * t588;
    let t592 = t12 * t19 * t2;
    let t594 = 6.0 * t592 * t27;
    let t595 = t21 * t579;
    let t596 = 1.0 / t595;
    (t582, t583, t586, t587, t588, t590, t594, t595, t596)
}
