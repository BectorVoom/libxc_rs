//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1767/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1767<F: Float>(t90509: F, t90511: F, t90514: F, t90578: F, t90580: F, t90582: F, t90585: F, t90588: F, t90592: F, t90594: F, t90597: F, t90599: F) -> F {
    let t90600 = t90509 + t90511 - t90514 + t90578 - t90580 - t90582 + t90585 + t90588 + t90592 - t90594 - t90597 + t90599;
    t90600
}
