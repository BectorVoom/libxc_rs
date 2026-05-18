//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 430/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk430<F: Float>(t2231: F, t27: F, t592: F, t596: F, t21: F, t25: F, t2219: F, t2221: F, t2223: F, t2226: F, t2228: F, t2230: F) -> (F, F, F) {
    let t2233 = F::new(30.0) * t2231 * t27;
    let t2235 = F::new(72.0) * t592 * t596;
    let t2236 = t21 * t21;
    let t2237 = F::new(1.0) / t2236;
    let t2239 = F::new(42.0) * t25 * t2237;
    let t2240 = t2219 - t2221 + t2223 + t2226 - t2228 + t2230 + t2233 - t2235 + t2239;
    (t2236, t2237, t2240)
}
