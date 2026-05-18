//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1168/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1168<F: Float>(t6193: F, t8217: F, t2260: F, t27668: F, t28465: F, t28508: F, t28547: F, t28727: F, t28856: F, t29366: F, t29370: F, t29373: F, t29381: F, t29384: F, t29387: F, t29397: F, t29402: F, t8213: F) -> (F, F) {
    let t29608 = t6193 * t8217;
    let t29622 = -F::new(0.61905925925925925925e-2) * t29366 - F::new(0.23214722222222222222e-2) * t29370 + F::new(0.17411041666666666666e-2) * t29373 + F::new(0.18534722222222222222e-2) * t29608 * t2260 - F::new(0.18534722222222222222e-2) * t28727 * t8213 + F::new(0.23214722222222222222e-2) * t28465 + F::new(0.30918233506944444444e-4) * t28856 - F::new(0.61905925925925925925e-2) * t28508 - F::new(0.17411041666666666666e-2) * t29381 + F::new(0.34822083333333333332e-2) * t29384 + F::new(0.92858888888888888886e-2) * t29387 - t27668 - F::new(0.23214722222222222222e-2) * t28547 - F::new(0.92858888888888888886e-2) * t29397 + F::new(0.17024129629629629629e-1) * t29402;
    (t29608, t29622)
}
