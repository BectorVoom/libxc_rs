//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1525/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1525<F: Float>(t290: F, t2925: F, t2967: F, t941: F, t2966: F, t307: F, t302: F) -> (F, F, F, F) {
    let t11387 = F::new(1.0) / t2925 / t290;
    let t11404 = t941 * t2967;
    let t11408 = F::new(1.0) / t2966 / t307;
    let t11409 = t302 * t11408;
    (t11387, t11404, t11408, t11409)
}
