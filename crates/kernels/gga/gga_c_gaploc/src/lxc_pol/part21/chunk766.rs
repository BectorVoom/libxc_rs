//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 766/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk766<F: Float>(t123: F, t7275: F, t734: F, t1858: F, t3294: F, t321: F, t935: F) -> (F, F, F, F, F) {
    let t7276 = t7275 * t123;
    let t7277 = t7276 * t734;
    let t7280 = t1858 * t3294;
    let t7281 = t7280 * t734;
    let t7284 = t321 * t935;
    (t7276, t7277, t7280, t7281, t7284)
}
