//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1056/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1056<F: Float>(t25343: F, t25351: F, t28805: F, t28811: F, t31590: F, t31594: F, t31598: F, t31603: F, t31606: F, t31610: F, t31616: F, t31621: F, t31626: F, t31630: F, t31633: F, t31637: F) -> (F,) {
    let t31929 = t31590 / 9.0 + t31594 / 18.0 + t31598 / 27.0 - t31603 / 3.0 - t31606 / 9.0 - 2.0 * t31610 - 4.0 / 9.0 * t28805 + 2.0 / 3.0 * t31616 + t31621 / 6.0 - t25343 - t25351 - 2.0 / 9.0 * t28811 - t31626 / 3.0 + t31630 / 3.0 + 2.0 / 3.0 * t31633 + 4.0 / 3.0 * t31637;
    (t31929,)
}
