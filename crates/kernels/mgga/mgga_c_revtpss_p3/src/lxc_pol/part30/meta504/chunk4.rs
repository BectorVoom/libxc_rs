//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1882/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1882<F: Float>(t2371: F, t25812: F, t25814: F, t25816: F, t25818: F, t25820: F, t25834: F, t26800: F, t26804: F, t27060: F, t670: F, t7586: F) -> F {
    let t27066 = F::new(2.0) * t2371 * t7586 + F::new(4.0) * t27060 * t670 + t25812 + t25814 + t25816 + t25818 + t25820 + t25834 + t26800 + F::new(2.0) * t26804;
    t27066
}
