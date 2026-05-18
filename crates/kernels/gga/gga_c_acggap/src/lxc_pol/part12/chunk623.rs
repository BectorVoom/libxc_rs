//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 623/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk623<F: Float>(t3132: F, t4353: F, t345: F, t3112: F, t3118: F, t3122: F, t3128: F, t3130: F, t3144: F, t3146: F, t3580: F, t3588: F, t3592: F) -> (F, F) {
    let t4833 = t3132 * t4353;
    let t4834 = t345 * t4833;
    let t4837 = -t3580 + F::new(0.489e0) * t3112 + F::new(0.12225e0) * t3118 - F::new(0.61125e-1) * t3122 - F::new(0.2445e0) * t3128 - F::new(0.978e0) * t3130 - t3588 - F::new(0.2282e1) * t3144 - F::new(0.22005e1) * t4834 + F::new(0.489e0) * t3146 + t3592;
    (t4834, t4837)
}
