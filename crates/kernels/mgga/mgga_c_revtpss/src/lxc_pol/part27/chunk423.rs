//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 423/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk423<F: Float>(t11: F, t14: F, t22: F, t584: F, t588: F, t20: F, t27: F, t12: F, t19: F, t592: F, t596: F, t21: F, t25: F, t2219: F, t2221: F, t2223: F) -> (F, F, F, F, F) {
    let t2224 = t11 * t14;
    let t2226 = 12.0 * t2224 * t22;
    let t2228 = 32.0 * t584 * t588;
    let t2230 = 20.0 * t20 * t27;
    let t2231 = t12 * t19;
    let t2233 = 30.0 * t2231 * t27;
    let t2235 = 72.0 * t592 * t596;
    let t2236 = t21 * t21;
    let t2237 = 1.0 / t2236;
    let t2239 = 42.0 * t25 * t2237;
    let t2240 = t2219 - t2221 + t2223 + t2226 - t2228 + t2230 + t2233 - t2235 + t2239;
    (t2224, t2231, t2236, t2237, t2240)
}
