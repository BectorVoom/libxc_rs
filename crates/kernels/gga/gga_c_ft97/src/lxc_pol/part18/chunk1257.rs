//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1257/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1257<F: Float>(t26199: F, t8392: F, t26203: F, t26166: F, t47667: F, t26358: F, t103510: F, t11449: F, t11473: F, t11827: F, t11855: F, t11960: F, t12012: F, t1307: F, t1901: F, t1909: F, t23265: F, t26245: F, t26349: F, t26423: F, t379: F, t39120: F, t446: F, t452: F, t46809: F, t47659: F, t488: F, t5631: F, t8506: F, t8557: F, t92072: F, t92074: F, t92088: F, t925: F) -> (F,) {
    let t103647 = 2.0 / 27.0 * t8392 * t26199;
    let t103649 = 4.0 / 27.0 * t8392 * t26203;
    let t103654 = t47667 * t26166;
    let t103686 = 2.0 / 27.0 * t8392 * t26358;
    let t103687 = -t1901 * t8557 * t23265 * t11449 / 9.0 + t103647 + t103649 - 2.0 / 9.0 * t1901 * t8557 * t26423 * t379 + 4.0 / 9.0 * t47659 * t103654 * t11855 + 2.0 / 9.0 * t1901 * t39120 * t23265 * t11827 + 4.0 / 9.0 * t47659 * t103510 * t11473 + t1901 * t1909 * t92088 * t925 / 9.0 + 2.0 / 9.0 * t1901 * t46809 * t5631 + t446 * t452 * t488 * t1307 * t11960 / 3.0 + 2.0 / 27.0 * t1901 * t26349 * t12012 + 16.0 / 27.0 * t92072 + t92074 / 9.0 + 2.0 / 9.0 * t1901 * t8506 * t26245 - t103686;
    (t103687,)
}
