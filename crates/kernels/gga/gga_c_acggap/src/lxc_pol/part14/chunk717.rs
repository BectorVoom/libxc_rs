//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 717/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk717<F: Float>(t2082: F, t7676: F, t2116: F, t950: F, t151: F) -> (F, F, F) {
    let t7677 = t7676 * t2082;
    let t7678 = F::new(0.12862205435420921092e-2) * t7677;
    let t7684 = t2116 * t950;
    let t7685 = t151 * t7684;
    (t7678, t7684, t7685)
}
