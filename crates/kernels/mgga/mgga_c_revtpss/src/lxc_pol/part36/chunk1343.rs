//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1343/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1343<F: Float>(t114752: F, t2035: F, t29499: F, t7898: F, t29495: F, t29506: F, t7937: F, t2014: F, t2034: F, t86791: F, t30112: F, t7935: F) -> (F, F, F, F, F, F, F) {
    let t114753 = t114752 * t2035;
    let t114755 = F::new(18.0) * t7898 * t29499;
    let t114757 = F::new(9.0) * t7898 * t29495;
    let t114759 = F::new(3.0) * t29506 * t7937;
    let t114765 = F::new(6.0) * t2014 * t2034 * t86791;
    let t114768 = F::new(3.0) * t7898 * t30112;
    let t114770 = F::new(3.0) * t29506 * t7935;
    (t114753, t114755, t114757, t114759, t114765, t114768, t114770)
}
