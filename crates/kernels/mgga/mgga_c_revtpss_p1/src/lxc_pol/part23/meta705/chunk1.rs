//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2457/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2457<F: Float>(t10022: F, t2453: F, t268: F, t39644: F, t546: F, t555: F, t8779: F, t1432: F, t4107: F, t9288: F, t10107: F, t3964: F, t9285: F) -> (F, F, F, F) {
    let t47429 = t2453 * t10022;
    let t47442 = F::cast_from(0.11638313500518478545e-4_f64) * t39644 * t546 * t555 * t8779 * t268;
    let t47444 = t1432 * t4107 * t9288;
    let t47450 = t3964 * t10107 * t9285;
    (t47429, t47442, t47444, t47450)
}
