//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 676/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk676<F: Float>(t1396: F, t1402: F, t1404: F, t1407: F, t153: F, t155: F, t400: F, t403: F, t5050: F, t5060: F, t5066: F, t5070: F, t5073: F, t5076: F, t519: F, t521: F, t917: F, t923: F, t926: F) -> (F,) {
    let t5079 = 6.0 * t1396 * t403 + 60.0 * t1402 * t5066 - 24.0 * t1402 * t5070 - 12.0 * t1402 * t5073 - 24.0 * t1404 * t5060 + 6.0 * t1407 * t400 + 3.0 * t153 * t5076 - t155 * t5050 - 12.0 * t519 * t923 + 3.0 * t519 * t926 + 3.0 * t521 * t917;
    (t5079,)
}
