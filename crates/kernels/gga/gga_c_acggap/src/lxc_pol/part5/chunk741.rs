//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 741/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk741<F: Float>(t1396: F, t1402: F, t1404: F, t1407: F, t153: F, t155: F, t1828: F, t1832: F, t1835: F, t400: F, t403: F, t519: F, t521: F, t6039: F, t6045: F, t6053: F, t6056: F, t6062: F, t6065: F) -> (F,) {
    let t6068 = 6.0 * t1396 * t521 + 60.0 * t1402 * t6053 - 24.0 * t1402 * t6056 - 12.0 * t1402 * t6062 - 24.0 * t1404 * t6045 + 6.0 * t1407 * t519 + 3.0 * t153 * t6065 - t155 * t6039 + 3.0 * t1828 * t403 - 12.0 * t1832 * t400 + 3.0 * t1835 * t400;
    (t6068,)
}
