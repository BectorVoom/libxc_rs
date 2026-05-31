//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1200/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1200<F: Float>(t10276: F, t22: F, t2224: F, t588: F, t27: F, t584: F, t20: F, t596: F, t12: F, t583: F, t2231: F, t2237: F, t592: F) -> (F, F, F, F, F, F, F) {
    let t10278 = F::cast_from(24.0_f64) * t10276 * t22;
    let t10279 = t2224 * t588;
    let t10281 = t584 * t27;
    let t10284 = F::cast_from(120.0_f64) * t20 * t596;
    let t10285 = t12 * t583;
    let t10287 = F::cast_from(120.0_f64) * t10285 * t27;
    let t10288 = t2231 * t596;
    let t10290 = t592 * t2237;
    (t10278, t10279, t10281, t10284, t10287, t10288, t10290)
}
