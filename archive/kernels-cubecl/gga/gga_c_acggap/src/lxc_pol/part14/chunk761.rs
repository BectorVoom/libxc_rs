//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 761/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk761<F: Float>(t7586: F, t8525: F, t7585: F, t2268: F, t7839: F, t2264: F, t7433: F, t1988: F, t2310: F, t1426: F, t2297: F, t429: F) -> (F, F, F, F, F, F) {
    let t8526 = t7586 * t8525;
    let t8527 = t7585 * t8526;
    let t8529 = t7839 * t2268;
    let t8531 = t7433 * t2264;
    let t8533 = t1988 * t2310;
    let t8536 = t1426 * t429 * t2297;
    (t8526, t8527, t8529, t8531, t8533, t8536)
}
