//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1333/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1333<F: Float>(t30112: F, t7898: F, t29506: F, t7935: F, t114401: F, t508: F, t651: F, t29583: F, t1450: F, t22809: F, t2014: F, t7237: F) -> (F, F, F, F, F) {
    let t114768 = F::new(3.0) * t7898 * t30112;
    let t114770 = F::new(3.0) * t29506 * t7935;
    let t114773 = F::new(2.0) * t651 * t508 * t114401;
    let t114775 = F::new(18.0) * t7898 * t29583;
    let t114776 = t1450 * t22809;
    let t114779 = F::new(3.0) * t2014 * t7237 * t114776;
    (t114768, t114770, t114773, t114775, t114779)
}
