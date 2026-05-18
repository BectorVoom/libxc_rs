//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 444/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk444<F: Float>(t2482: F, t27: F, t823: F, t136: F, t826: F, t737: F) -> (F, F, F, F) {
    let t2484 = t2482 * t823 * t27;
    let t2485 = t826 * t136;
    let t2490 = t737 * t737;
    let t2491 = F::new(1.0) / t2490;
    (t2484, t2485, t2490, t2491)
}
