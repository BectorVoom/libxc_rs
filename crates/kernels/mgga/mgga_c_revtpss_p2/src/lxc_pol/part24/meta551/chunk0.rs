//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1638/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1638<F: Float>(t6079: F, t1544: F, t1583: F, t18850: F, t1940: F, t198: F, t207: F, t23148: F, t2403: F, t40076: F, t40079: F, t40194: F, t40198: F, t41154: F, t4541: F, t4546: F, t5966: F, t765: F, t77357: F, t77373: F, t87543: F, t87676: F, t87677: F, t87678: F, t87679: F) -> F {
    let t87970 = t6079 * t6079;
    let t87987 = -F::new(6.0) * t198 * t207 * t41154 * t87970 + F::new(24.0) * t1544 * t2403 * t77373 - F::new(4.0) * t1583 * t1940 * t77357 + F::new(36.0) * t18850 * t4541 * t5966 + F::new(3.0) * t198 * t765 * t87543 + F::new(12.0) * t23148 * t2403 * t4546 + t40076 - t40079 + t40194 + t40198 + t87676 + t87677 - t87678 - t87679;
    t87987
}
