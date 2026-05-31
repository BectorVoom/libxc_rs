//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1421/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1421<F: Float>(t13254: F, t13256: F, t1456: F, t1458: F, t1464: F, t18184: F, t18186: F, t18219: F, t1914: F, t1921: F, t22533: F, t22536: F, t22542: F, t22571: F, t575: F, t5790: F, t5808: F, t6937: F, t6951: F) -> F {
    let tv3rho32 = t1456 * t6951 + t1458 * t22571 + t1464 * t6937 + F::cast_from(2.0_f64) * t1914 * t5808 + F::cast_from(2.0_f64) * t1921 * t5790 + t22533 * t575 + t13254 + t13256 + t18184 + t18186 + t18219 + F::cast_from(2.0_f64) * t22536 + t22542;
    tv3rho32
}
