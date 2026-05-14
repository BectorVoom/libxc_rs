//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 842/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk842<F: Float>(t33: F, t265: F, t502: F, t27754: F, t1469: F, t2003: F, t27821: F, t4186: F, t57: F, t606: F, t7215: F, t7877: F, t27762: F, t196: F, t197: F, t5528: F, t2035: F, t7313: F, t7898: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t27822 = piecewise3(t503, 0.0, t27754);
    let t27829 = piecewise3(t400, t27821, -t7215 * t1469 / 2.0 - t2003 * t4186 / 2.0 + t27822 * t57 / 2.0 - t7877 * t606 / 2.0);
    let t27830 = t27762 + t27829;
    let t27833 = t5528 * t196 * t197;
    let t27834 = t27833 * t2035;
    let t27835 = t7898 * t7313;
    (t27830, t27833, t27834, t27835)
}
