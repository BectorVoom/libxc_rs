//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 789/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk789<F: Float>(t12: F, t583: F, t27: F, t2231: F, t596: F, t2237: F, t592: F, t2236: F, t3: F, t25: F, t10271: F, t10273: F, t10275: F, t10278: F, t10280: F, t10282: F, t10284: F) -> (F,) {
    let t10285 = t12 * t583;
    let t10287 = 120.0 * t10285 * t27;
    let t10288 = t2231 * t596;
    let t10289 = 540.0 * t10288;
    let t10290 = t592 * t2237;
    let t10291 = 756.0 * t10290;
    let t10292 = t2236 * t3;
    let t10293 = 1.0 / t10292;
    let t10295 = 336.0 * t25 * t10293;
    let t10296 = -t10271 + t10273 - t10275 + t10278 - t10280 + t10282 - t10284 + t10287 - t10289 + t10291 - t10295;
    (t10296,)
}
