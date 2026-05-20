//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1791/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1791<F: Float>(t25237: F, t5989: F, t5993: F, t7045: F, t5985: F, t7025: F, t6019: F, t7038: F, t6030: F, t1558: F, t1579: F, t231: F) -> (F, F, F, F, F, F) {
    let t29623 = t25237 * t5989;
    let t29627 = t7045 * t5993;
    let t29629 = t7025 * t5985;
    let t29631 = t7038 * t6019;
    let t29633 = t7045 * t6030;
    let t29682 = t1579 * t1558 * t231;
    (t29623, t29627, t29629, t29631, t29633, t29682)
}
