//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1111/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1111<F: Float>(t99315: F, t99317: F, t99457: F, t99467: F, t99509: F, t99524: F, t99534: F, t99537: F, t99607: F, t2399: F, t6349: F, t89: F, t10696: F, t1495: F, t28489: F, t29451: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t99736 = 2.0 / 27.0 * t99315;
    let t99737 = 4.0 / 27.0 * t99317;
    let t99780 = t99457 / 9.0;
    let t99782 = 4.0 / 9.0 * t99467;
    let t99795 = 2.0 / 9.0 * t99509;
    let t99799 = 14.0 / 81.0 * t99524;
    let t99801 = 28.0 / 81.0 * t99534;
    let t99802 = 4.0 / 9.0 * t99537;
    let t99825 = 8.0 / 9.0 * t99607;
    let t99867 = t89 * t2399 * t6349;
    let t99918 = t1495 * t10696;
    let t107750 = 2.0 * t28489;
    let t107751 = 2.0 * t29451;
    (t99736, t99737, t99780, t99782, t99795, t99799, t99801, t99802, t99825, t99867, t99918, t107750, t107751)
}
