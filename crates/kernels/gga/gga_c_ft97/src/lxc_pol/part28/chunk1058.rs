//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1058/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1058<F: Float>(t137163: F, t137172: F, t137180: F, t145038: F, t145042: F, t145045: F, t145048: F, t145051: F, t145055: F, t145058: F, t145061: F, t145588: F, t145590: F, t145595: F, t145598: F, t145601: F) -> F {
    let t145603 = t145038 + t145042 / F::new(4.0) - t145045 - F::new(3.0) * t145048 - F::new(2.0) / F::new(3.0) * t145051 + F::new(2.0) / F::new(9.0) * t137163 + F::new(4.0) / F::new(3.0) * t145055 - F::new(4.0) / F::new(9.0) * t145058 + t145061 / F::new(6.0) - t145588 + t145590 / F::new(3.0) + F::new(4.0) / F::new(3.0) * t137172 - F::new(8.0) / F::new(3.0) * t137180 + F::new(2.0) * t145595 - F::new(4.0) / F::new(3.0) * t145598 + F::new(4.0) / F::new(3.0) * t145601;
    t145603
}
