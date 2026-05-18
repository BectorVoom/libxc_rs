//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1165/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1165<F: Float>(t139453: F, t139485: F, t140041: F, t140042: F, t148593: F, t148597: F, t148601: F, t148604: F, t148607: F, t148611: F, t148616: F, t148621: F, t148625: F, t148629: F, t148632: F, t148636: F) -> F {
    let t148844 = F::new(2.0) * t148593 + F::new(4.0) * t148597 - F::new(6.0) * t148601 - F::new(2.0) / F::new(3.0) * t148604 - F::new(4.0) / F::new(3.0) * t148607 + t139453 / F::new(3.0) + F::new(2.0) * t148611 + t148616 + t148621 / F::new(4.0) - F::new(12.0) * t148625 + t139485 / F::new(9.0) + t148629 + t148632 / F::new(3.0) - t148636 - t140041 + t140042;
    t148844
}
