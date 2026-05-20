//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3173/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3173<F: Float>(t16943: F, t3379: F, t43762: F, t43771: F, t43773: F, t43781: F, t43783: F, t43785: F, t43787: F, t44039: F, t44040: F, t56151: F, t56155: F) -> (F, F) {
    let t58481 = F::new(3.0) * t3379 * t16943;
    let t58491 = -F::cast_from(0.24342716049382716049e-1_f64) * t43762 - F::cast_from(0.73028148148148148149e0_f64) * t43771 + F::cast_from(0.10954222222222222222e0_f64) * t43773 + F::cast_from(0.27385555555555555556e0_f64) * t43781 + F::cast_from(0.54771111111111111111e0_f64) * t43783 - F::cast_from(0.54771111111111111111e-1_f64) * t43785 - F::cast_from(0.32862666666666666666e0_f64) * t43787 + t44039 + t44040 - F::cast_from(0.71752000000000000002e1_f64) * t56151 + F::new(0.17938e1) * t56155;
    (t58481, t58491)
}
