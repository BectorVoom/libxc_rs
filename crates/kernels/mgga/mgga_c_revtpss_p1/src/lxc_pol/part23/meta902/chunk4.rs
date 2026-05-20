//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2883/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2883<F: Float>(t1544: F, t4537: F, t14353: F, t18268: F, t2403: F, t40167: F, t40171: F, t40184: F, t4433: F, t4541: F, t50884: F, t5962: F, t77024: F, t77025: F, t77026: F, t77027: F, t77028: F) -> (F, F) {
    let t77441 = t1544 * t4537;
    let t77455 = F::new(9.0) * t14353 * t2403 * t5962 - F::new(18.0) * t18268 * t4433 * t4541 + t40167 - t40171 - t40184 + t50884 - t77024 + t77025 + t77026 + t77027 - t77028;
    (t77441, t77455)
}
