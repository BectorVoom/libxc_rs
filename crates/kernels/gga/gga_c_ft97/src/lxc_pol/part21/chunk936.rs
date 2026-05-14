//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 936/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk936<F: Float>(t29661: F, t5674: F, t4454: F, t5675: F, t7793: F, t23114: F, t25913: F, t25940: F, t25946: F, t25948: F, t29623: F, t29627: F, t29631: F, t29635: F, t29639: F, t29643: F, t29647: F, t29654: F, t29658: F) -> (F, F, F, F) {
    let t29662 = t5674 * t29661;
    let t29665 = t7793 * t5675 * t4454;
    let t29666 = t5674 * t29665;
    let t29668 = -4.0 / 9.0 * t25913 + t29623 / 9.0 + 2.0 / 27.0 * t29627 + 2.0 / 9.0 * t29631 - t29635 / 6.0 - t29639 / 9.0 - 2.0 / 9.0 * t29643 - t29647 / 18.0 + 2.0 / 9.0 * t25940 - 2.0 / 27.0 * t25946 - t25948 / 27.0 - 2.0 / 9.0 * t29654 - t23114 + t29658 / 9.0 + t29662 / 18.0 + t29666 / 27.0;
    (t29662, t29665, t29666, t29668)
}
