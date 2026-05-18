//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1427/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1427<F: Float>(t13254: F, t13256: F, t1456: F, t1458: F, t1464: F, t18178: F, t18184: F, t18186: F, t18217: F, t1914: F, t1921: F, t4154: F, t4168: F, t575: F, t5790: F, t5808: F, t9263: F, t9265: F, t9267: F) -> F {
    let tv3rho31 = F::new(2.0) * t1456 * t5808 + t1458 * t18217 + F::new(2.0) * t1464 * t5790 + t18178 * t575 + t1914 * t4168 + t1921 * t4154 + t13254 + t13256 + t18184 + t18186 + t9263 + F::new(2.0) * t9265 + t9267;
    tv3rho31
}
