//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 803/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk803<F: Float>(t8742: F, t8744: F, t7465: F, t7466: F, t7469: F, t7479: F, t7481: F, t7485: F, t7489: F, t7497: F, t7500: F, t8184: F, t8185: F, t8740: F, t8748: F) -> F {
    let t9277 = F::new(0.4584375e-1) * t8742;
    let t9278 = F::new(0.305625e-1) * t8744;
    let t9280 = t7465 - F::new(0.56606566121287473723e-2) * t7466 + t7469 + F::new(0.1048269742986805069e-2) * t7479 - F::new(0.62896184579208304138e-3) * t7481 + t7485 + t7489 - t7497 + F::new(0.62896184579208304138e-3) * t7500 + F::new(0.62896184579208304138e-3) * t8740 + t9277 + t9278 + t8184 - t8185 - F::new(0.7640625e-2) * t8748;
    t9280
}
