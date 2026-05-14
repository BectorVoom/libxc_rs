//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1382/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1382<F: Float>(t193: F, t25027: F, t5337: F, t6260: F, t852: F, t1882: F, t31569: F, t127759: F, t127763: F, t127767: F, t127770: F, t127773: F, t127776: F, t127779: F, t127781: F, t127784: F) -> (F, F, F) {
    let t127789 = t25027 * t193 * t852 * t6260 * t5337;
    let t127791 = t1882 * t31569;
    let t127792 = 4.0 / 9.0 * t127791;
    let t127793 = 2.0 / 3.0 * t127759 + 24.0 * t127763 + t127767 / 3.0 - t127770 / 3.0 - 2.0 / 3.0 * t127773 - 2.0 / 3.0 * t127776 + t127779 + 2.0 * t127781 - 4.0 / 3.0 * t127784 - 3.0 / 8.0 * t127789 + t127792;
    (t127789, t127791, t127793)
}
