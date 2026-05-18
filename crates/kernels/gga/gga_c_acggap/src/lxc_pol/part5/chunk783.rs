//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 783/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk783<F: Float>(t150: F, t6004: F, t6010: F, t6019: F, t6036: F, t519: F, t94: F, t1024: F, t1713: F, t301: F, t1298: F, t1403: F) -> (F, F, F, F, F) {
    let t6039 = (t6004 + t6010 + t6019 + t6036) * t150;
    let t6045 = t519 * t94;
    let t6052 = t1024 * t1713;
    let t6053 = t6052 * t301;
    let t6056 = t1403 * t1298;
    (t6039, t6045, t6052, t6053, t6056)
}
