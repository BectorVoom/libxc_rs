//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2032/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2032<F: Float>(t110242: F, t110261: F, t110281: F, t110306: F, t110330: F, t110348: F, t110365: F, t110466: F, t110499: F, t110519: F, t110551: F, t110576: F, t110607: F, t110635: F, t110665: F, t110694: F) -> F {
    let t110698 = t110242 + t110261 + t110281 + t110306 + t110330 + t110348 + t110365 + t110466 + t110499 + t110519 + t110551 + t110576 + t110607 + t110635 + t110665 + t110694;
    t110698
}
