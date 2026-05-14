//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 767/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk767<F: Float>(t3238: F, t452: F, t7229: F, t1332: F, t6454: F, t488: F, t1339: F, t1871: F, t6469: F, t447: F, t7288: F, t925: F, t32433: F, t32463: F, t32469: F, t34624: F, t34629: F, t34634: F, t34637: F, t34640: F, t446: F) -> (F, F, F, F, F, F) {
    let t34644 = t452 * t3238 * t7229;
    let t34647 = t6454 * t1332;
    let t34649 = t452 * t488 * t34647;
    let t34653 = t1871 * t1339 * t6469;
    let t34657 = t447 * t7288 * t925;
    let t34660 = t32433 + 2.0 / 3.0 * t446 * t34624 + t446 * t34629 / 3.0 - 2.0 / 3.0 * t446 * t34634 - 2.0 * t446 * t34637 + 4.0 / 3.0 * t446 * t34640 + t32463 + 2.0 / 3.0 * t446 * t34644 + 2.0 / 3.0 * t446 * t34649 + 4.0 / 3.0 * t446 * t34653 + t32469 - t446 * t34657 / 9.0;
    (t34644, t34647, t34649, t34653, t34657, t34660)
}
