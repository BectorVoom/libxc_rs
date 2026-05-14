//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 579/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk579<F: Float>(t5468: F, t898: F, t900: F, t2265: F, t2912: F, t4332: F, t4350: F, t5442: F, t5446: F, t5450: F, t5454: F, t5459: F, t631: F, t332: F, t113: F, t1273: F) -> (F, F, F, F, F) {
    let t5470 = t898 * t900 * t5468;
    let t5473 = -t2912 - 2.0 / 9.0 * t4332 - 2.0 / 3.0 * t4350 + t631 * t5442 / 18.0 - 2.0 / 3.0 * t2265 * t5446 - t631 * t5450 / 3.0 + t631 * t5454 / 6.0 - 3.0 / 2.0 * t631 * t5459 + t631 * t5470 / 2.0;
    let t5474 = t5473 * t332;
    let t5475 = t5474 * t113;
    let t5478 = t1273 * t1273;
    (t5470, t5473, t5474, t5475, t5478)
}
