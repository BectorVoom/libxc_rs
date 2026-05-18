//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1306/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1306<F: Float>(t11283: F, t11297: F, t11611: F, t33091: F, t33093: F, t33094: F, t33095: F, t33096: F, t33097: F, t33098: F, t33099: F, t33100: F, t33101: F, t36085: F, t36086: F, t7: F) -> F {
    let t36089 = F::new(4.0) * t11283;
    let t36090 = F::new(2.0) * t11297;
    let tv4rho2sigma20 = t33091 + F::new(2.0) * t11611 + t33093 - t33094 + t33095 - t33096 + t33097 + t33098 - t33099 - t33100 + t33101 + t7 * (t36085 + t36086) - t36089 - t36090;
    tv4rho2sigma20
}
