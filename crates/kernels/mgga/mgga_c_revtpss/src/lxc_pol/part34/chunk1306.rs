//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1306/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1306<F: Float>(t33: F, t265: F, t502: F, t114149: F, t114199: F, t114089: F, t1469: F, t2003: F, t22671: F, t29978: F, t57: F, t5825: F, t7877: F, t2014: F, t30111: F, t5542: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t114200 = t114149 + t114199;
    let t114201 = piecewise3::<F>(t503, F::new(0.0), t114089);
    let t114211 = piecewise3::<F>(t400, t114200, t114201 * t57 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t29978 * t1469 - F::new(3.0) / F::new(2.0) * t7877 * t5825 - t2003 * t22671 / F::new(2.0));
    let t114216 = F::new(3.0) * t2014 * t30111 * t5542;
    (t114211, t114216)
}
