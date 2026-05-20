//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2881/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2881<F: Float>(t5966: F, t890: F, t18435: F, t18498: F, t39989: F, t40150: F, t4541: F, t4546: F, t4556: F, t50098: F, t77007: F, t77008: F, t77009: F, t77010: F, t77011: F) -> F {
    let t77408 = t5966 * t890;
    let t77412 = F::new(18.0) * t18435 * t4541 * t4546 + F::new(36.0) * t18498 * t4541 * t4546 - F::new(18.0) * t4541 * t4556 * t77408 - t39989 + t40150 + t50098 + t77007 + t77008 + t77009 + t77010 - t77011;
    t77412
}
