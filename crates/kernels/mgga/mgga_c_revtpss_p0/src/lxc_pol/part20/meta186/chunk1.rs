//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 933/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk933<F: Float>(t1376: F, t9789: F, t235: F, t4086: F, t2453: F) -> (F, F, F) {
    let t9791 = F::cast_from(0.11294745624363664198e-6_f64) * t9789 * t1376;
    let t9792 = t4086 * t235;
    let t9793 = t2453 * t9792;
    (t9791, t9792, t9793)
}
