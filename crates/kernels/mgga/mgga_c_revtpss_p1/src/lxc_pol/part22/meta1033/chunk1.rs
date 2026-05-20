//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3617/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3617<F: Float>(t58207: F, t68454: F, t68529: F, t68532: F, t68535: F, t68538: F, t68540: F, t68543: F, t68546: F, t68548: F, t68550: F, t68553: F, t68556: F, t68559: F, t68561: F) -> F {
    let t68564 = F::new(0.44152e0) * t68529 - F::cast_from(0.8585111111111111111e-1_f64) * t68532 + F::new(0.33114e0) * t68535 - F::cast_from(0.49057777777777777777e-1_f64) * t58207 - F::new(0.44152e0) * t68538 - F::new(0.66228e0) * t68540 + F::new(0.16557e0) * t68543 + F::new(0.49671e0) * t68546 + F::cast_from(0.73586666666666666667e-1_f64) * t68548 + F::new(0.22076e0) * t68550 - F::new(0.5519e-1) * t68553 + F::cast_from(0.36793333333333333333e-1_f64) * t68556 + F::new(0.776775e1) * t68559 - F::new(0.16504875e0) * t68561 - F::cast_from(0.80513333333333333333e0_f64) * t68454;
    t68564
}
