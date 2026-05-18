//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 685/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk685<F: Float>(t13253: F, t2343: F, t11167: F, t2325: F, t883: F, t882: F, t11254: F, t874: F) -> (F, F, F, F) {
    let t13254 = t2343 * t13253;
    let t13258 = t2325 * t883 * t11167;
    let t13259 = t882 * t13258;
    let t13260 = F::new(0.11856252764865062333e-2) * t13259;
    let t13261 = t11254 * t874;
    (t13254, t13258, t13260, t13261)
}
