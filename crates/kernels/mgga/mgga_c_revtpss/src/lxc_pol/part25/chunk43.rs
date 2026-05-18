//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 43/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk43<F: Float>(t41: F, t30: F, t53: F, t33: F, rho0: F, rho1: F, tau0: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t96 = F::new(1.0) / t41 / rho0;
    let t97 = tau0 * t96;
    let t98 = t30 / F::new(2.0);
    let t99 = pow_1_3::<f64>(t98);
    let t100 = t99 * t99;
    let t101 = t100 * t98;
    let t104 = F::new(1.0) / t53 / rho1;
    let t105 = tau1 * t104;
    let t106 = t33 / F::new(2.0);
    let t107 = pow_1_3::<f64>(t106);
    let t108 = t107 * t107;
    let t109 = t108 * t106;
    let t111 = t101 * t97 + t105 * t109;
    let t112 = F::new(1.0) / t111;
    (t97, t98, t99, t100, t101, t105, t106, t107, t108, t111, t112)
}
