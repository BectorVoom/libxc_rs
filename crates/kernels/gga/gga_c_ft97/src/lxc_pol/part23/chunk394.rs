//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 394/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk394<F: Float>(t4370: F, t898: F, t900: F, t2265: F, t2912: F, t2913: F, t2915: F, t3628: F, t4332: F, t4335: F, t4339: F, t4343: F, t4347: F, t4350: F, t4354: F, t4359: F, t631: F) -> (F, F) {
    let t4372 = t898 * t900 * t4370;
    let t4375 = -t2912 - t2913 / 9.0 - t2915 / 3.0 - t4332 / 9.0 + t2265 * t4335 / 18.0 - t2265 * t4339 / 3.0 - t2265 * t4343 / 3.0 - t3628 * t4347 / 3.0 - t4350 / 3.0 - t2265 * t4354 / 3.0 - 3.0 / 2.0 * t631 * t4359 + t631 * t4372 / 2.0;
    (t4372, t4375)
}
