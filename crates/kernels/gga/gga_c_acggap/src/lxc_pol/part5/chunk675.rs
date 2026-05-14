//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 675/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk675<F: Float>(t150: F, t5019: F, t5020: F, t5029: F, t5047: F, t400: F, t94: F, t1024: F, t495: F, t922: F, t1298: F, t420: F, t301: F, t1403: F, t839: F, t402: F, t4099: F) -> (F, F, F, F, F, F, F) {
    let t5050 = (t5019 + t5020 + t5029 + t5047) * t150;
    let t5060 = t400 * t94;
    let t5065 = t1024 * t495;
    let t5066 = t5065 * t922;
    let t5069 = t420 * t1298;
    let t5070 = t5069 * t301;
    let t5073 = t1403 * t839;
    let t5076 = t402 * t4099;
    (t5050, t5060, t5065, t5066, t5070, t5073, t5076)
}
