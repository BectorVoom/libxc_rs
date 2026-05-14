//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1338/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1338<F: Float>(t147: F, t118536: F, t121676: F, t100044: F, t100045: F, t118509: F, t13: F, t30032: F, t30585: F) -> (F,) {
    let t148 = 10000000.0 <= t147;
    let t121678 = piecewise3(t148, 0.0, t118536 + t121676);
    let tv4rho3sigma6 = t100044 + t100045 + t30032 + t30585 + t13 * (t118509 + t121678);
    (tv4rho3sigma6,)
}
