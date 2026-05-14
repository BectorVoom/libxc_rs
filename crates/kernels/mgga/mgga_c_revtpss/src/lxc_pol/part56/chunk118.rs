//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 118/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk118<F: Float>(t406: F, t409: F, t412: F, t416: F, t439: F, t300: F, t424: F, t426: F, t435: F) -> (F, F, F, F, F, F) {
    let t444 = 0.51785e1 * t409 + 0.905775e0 * t406 + 0.1100325e0 * t412 + 0.1241775e0 * t416;
    let t447 = 1.0 + 0.29608749977793437516e2 / t444;
    let t448 = f64::ln(t447);
    let t449 = t439 * t448;
    let t452 = t300 * (-0.310907e-1 * t426 * t435 + t424 - 0.19751673498613801407e-1 * t449);
    let t454 = 0.19751673498613801407e-1 * t300 * t449;
    let t456 = 1.0 + 0.25e-1 * t406;
    (t444, t447, t448, t452, t454, t456)
}
