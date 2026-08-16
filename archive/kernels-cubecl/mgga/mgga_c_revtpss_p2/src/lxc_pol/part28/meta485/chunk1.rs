//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1845/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1845<F: Float>(t33: F, t265: F, t502: F, t25743: F, t2003: F, t2258: F, t25791: F, t57: F, t606: F, t7215: F, t25751: F, t4135: F, t4147: F, t2034: F, t2014: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t25792 = piecewise3::<F>(t503, F::cast_from(0.0_f64), t25743);
    let t25799 = piecewise3::<F>(t400, t25791, t25792 * t57 / F::cast_from(2.0_f64) - t7215 * t606 - t2003 * t2258 / F::cast_from(2.0_f64));
    let t25800 = t25751 + t25799;
    let t25802 = t4147 * t4135;
    let t25803 = t2034 * t25802;
    let t25804 = t2014 * t25803;
    (t25792, t25800, t25802, t25803, t25804)
}
