//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1075/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1075<F: Float>(t137198: F, t137205: F, t137213: F, t137215: F, t137219: F, t137229: F, t145667: F, t145669: F, t145673: F, t145676: F, t145681: F, t145684: F, t145687: F, t145691: F, t145695: F, t145699: F) -> F {
    let t145906 = t145667 / F::new(3.0) + t137198 + t137205 - t137213 + F::new(4.0) / F::new(9.0) * t145669 + F::new(2.0) / F::new(27.0) * t145673 - F::new(8.0) / F::new(9.0) * t145676 + t137215 / F::new(9.0) - t137219 - t137229 / F::new(27.0) - F::new(4.0) / F::new(9.0) * t145681 + F::new(2.0) / F::new(3.0) * t145684 - F::new(2.0) / F::new(9.0) * t145687 + F::new(4.0) / F::new(3.0) * t145691 + F::new(4.0) / F::new(3.0) * t145695 - F::new(2.0) * t145699;
    t145906
}
