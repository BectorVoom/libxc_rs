//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1212/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1212<F: Float>(t120108: F, t120120: F, t120139: F, t122004: F, t122008: F, t122009: F, t122010: F, t122015: F, t126365: F, t126376: F, t32445: F, t34075: F) -> F {
    let t127847 = F::cast_from(0.25702851531048074406e-1_f64) * t122004 - F::cast_from(0.17135921299530705785e1_f64) * t34075 * t32445 + t120108 - t122008 + t122009 - t122010 - t120120 - t122015 - F::cast_from(0.56468933516960933999e-3_f64) * t126365 + t120139 + F::cast_from(0.37645955677973955999e-4_f64) * t126376;
    t127847
}
