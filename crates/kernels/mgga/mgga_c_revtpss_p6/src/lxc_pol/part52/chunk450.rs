//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 450/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk450<F: Float>(t143: F, t680: F, t130: F, t700: F, t701: F) -> (F, F) {
    let t2563 = t680 * t143;
    let t2564 = F::new(1.0) / t2563;
    let t2565 = t130 * t2564;
    let t2566 = t700 * t700;
    let t2567 = t2566 * t701;
    let t2569 = F::new(2.0) * t2565 * t2567;
    (t2566, t2569)
}
