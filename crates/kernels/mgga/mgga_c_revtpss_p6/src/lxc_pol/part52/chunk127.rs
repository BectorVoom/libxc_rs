//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 127/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk127<F: Float>(t225: F, t487: F, t473: F, t460: F) -> (F, F, F, F, F) {
    let t488 = t487 * t225;
    let t489 = t225 * t473;
    let t490 = t489 * t487;
    let t493 = F::new(1.0) + F::cast_from(0.65854491829355115987e0_f64) * t460 * t490;
    let t494 = F::new(1.0) / t493;
    (t488, t489, t490, t493, t494)
}
