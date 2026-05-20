//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3670/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3670<F: Float>(t58207: F, t68454: F, t68529: F, t68532: F, t68535: F, t68538: F, t68540: F, t68543: F, t68546: F, t68548: F, t68550: F, t68553: F, t68556: F, t68559: F, t68561: F) -> F {
    let t69329 = F::cast_from(0.55570666666666666666e0_f64) * t68529 - F::cast_from(0.10805407407407407407e0_f64) * t68532 + F::new(0.41678e0) * t68535 - F::cast_from(0.61745185185185185187e-1_f64) * t58207 - F::cast_from(0.55570666666666666667e0_f64) * t68538 - F::cast_from(0.83356000000000000001e0_f64) * t68540 + F::new(0.20839e0) * t68543 + F::new(0.62517e0) * t68546 + F::cast_from(0.92617777777777777779e-1_f64) * t68548 + F::cast_from(0.27785333333333333334e0_f64) * t68550 - F::cast_from(0.69463333333333333334e-1_f64) * t68553 + F::cast_from(0.46308888888888888889e-1_f64) * t68556 + F::new(0.10589175e2) * t68559 - F::new(0.6311625e0) * t68561 - F::cast_from(0.13772666666666666667e1_f64) * t68454;
    t69329
}
