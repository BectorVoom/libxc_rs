//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 601/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk601<F: Float>(t3112: F, t3128: F, t3144: F, t3579: F, t3580: F, t3588: F, t3592: F, t4809: F, t4812: F, t4814: F, t4817: F, t5667: F) -> F {
    let t5673 = -t4809 - F::new(0.2445e0) * t4812 - F::new(0.2282e1) * t4814 - t4817 + t3579 - t3580 + F::new(0.2445e0) * t3112 - F::new(0.12225e0) * t3128 - t3588 - F::new(0.1141e1) * t3144 + t3592;
    let t5674 = t5667 + t5673;
    t5674
}
