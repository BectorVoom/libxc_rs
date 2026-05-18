//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 707/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk707<F: Float>(t218: F, t7021: F, t816: F, t1941: F, t228: F, t240: F, t64: F) -> (F, F, F) {
    let t7023 = t7021 * t218 * t816;
    let t7024 = F::new(7.0) / F::new(288.0) * t7023;
    let t7025 = t1941 * t228;
    let t7028 = t64 * t240;
    (t7024, t7025, t7028)
}
