//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 174/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk174<F: Float>(t687: F, t689: F, t693: F, t698: F, t146: F, t682: F) -> (F, F, F, F) {
    let t700 = -F::new(0.632975e0) * t687 - F::new(0.29896666666666666667e0) * t689 - F::new(0.1023875e0) * t693 - F::new(0.82156666666666666667e-1) * t698;
    let t701 = F::new(1.0) / t146;
    let t702 = t700 * t701;
    let t704 = F::new(1.0) * t682 * t702;
    (t700, t701, t702, t704)
}
