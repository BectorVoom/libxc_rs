//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1349/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1349<F: Float>(t2612: F, t40207: F, t190: F, t2611: F, t39449: F, t40076: F, t40079: F, t40184: F, t40187: F, t40190: F, t40194: F, t40198: F, t40202: F, t40204: F, t40206: F) -> (F, F, F) {
    let t40209 = F::new(72.0) * t40207 * t2612;
    let t40212 = F::new(36.0) * t2611 * t190 * t39449;
    let t40213 = -t40184 + t40187 + t40190 + t40076 - t40079 + t40194 + t40198 + t40202 + t40204 - t40206 + t40209 + t40212;
    (t40209, t40212, t40213)
}
