//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1489/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1489<F: Float>(t31032: F, t31434: F, t117461: F, t31447: F, t2357: F, t55: F, t116929: F, t8402: F, t116926: F, t8395: F, t2289: F, t8399: F) -> (F, F, F, F, F, F) {
    let t117920 = F::new(50.0) / F::new(27.0) * t31032 * t31434;
    let t117927 = t117461 * t31447;
    let t117932 = t55 * t2357;
    let t117936 = t116929 * t8402;
    let t117938 = t116926 * t8395;
    let t117940 = t2289 * t8399;
    (t117920, t117927, t117932, t117936, t117938, t117940)
}
