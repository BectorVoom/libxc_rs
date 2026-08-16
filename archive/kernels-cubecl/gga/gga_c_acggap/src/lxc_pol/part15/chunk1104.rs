//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1104/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1104<F: Float>(t1980: F, t39066: F, t5011: F, t7458: F, t2001: F, t5821: F, t1998: F, t5569: F, t1967: F, t9554: F, t6161: F, t7561: F) -> (F, F, F, F, F) {
    let t39069 = t1980 * t7458 * t5011 * t39066;
    let t39071 = t2001 * t5821;
    let t39073 = t1998 * t5569;
    let t39075 = t1967 * t9554;
    let t39077 = t7561 * t6161;
    (t39069, t39071, t39073, t39075, t39077)
}
