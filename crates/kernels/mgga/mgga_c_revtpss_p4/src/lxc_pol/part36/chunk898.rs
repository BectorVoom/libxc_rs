//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 898/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk898<F: Float>(t18423: F, t2674: F, t125: F, t5977: F, t221: F, t2485: F, t6022: F, t10850: F, t14718: F, t6035: F, t2662: F, t2661: F) -> (F, F, F, F, F, F) {
    let t18424 = t2674 * t18423;
    let t18426 = t125 * t5977;
    let t18432 = t2485 * t221 * t6022;
    let t18433 = t10850 * t18432;
    let t18440 = t14718 * t6035;
    let t18441 = t2662 * t18440;
    let t18442 = t2661 * t18441;
    (t18424, t18426, t18432, t18433, t18440, t18442)
}
