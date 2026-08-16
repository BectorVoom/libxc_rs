//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3668/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3668<F: Float>(t58145: F, t58147: F, t68470: F, t68473: F, t68476: F, t68479: F, t68481: F, t68484: F, t68486: F, t68488: F, t68490: F, t68493: F, t68495: F, t68497: F) -> F {
    let t69296 = -F::cast_from(0.6618234375e1_f64) * t68470 + F::cast_from(0.264729375e1_f64) * t68473 + F::cast_from(0.2366859375e0_f64) * t68476 - F::cast_from(0.157790625e0_f64) * t68479 - F::cast_from(0.3529725e1_f64) * t68481 - F::cast_from(0.3529725e1_f64) * t68484 - F::cast_from(0.17648625e1_f64) * t68486 - F::cast_from(0.157790625e0_f64) * t68488 + F::cast_from(0.6311625e0_f64) * t68490 + F::cast_from(0.6311625e0_f64) * t68493 + F::cast_from(0.31558125e0_f64) * t68495 + F::cast_from(0.264729375e1_f64) * t68497 + F::cast_from(0.4630888888888888889e0_f64) * t58145 - F::cast_from(0.13892666666666666667e0_f64) * t58147;
    t69296
}
