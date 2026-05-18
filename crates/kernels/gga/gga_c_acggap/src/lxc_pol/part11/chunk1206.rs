//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1206/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1206<F: Float>(t2333: F, t848: F, t2342: F, t30005: F, t2131: F, t2132: F, t2331: F, t847: F, t7994: F, t8998: F, t1221: F, t525: F) -> (F, F, F, F, F) {
    let t36531 = t848 * t2333;
    let t36533 = t30005 * t2342;
    let t36541 = t2131 * t2132 * t2331 * t847;
    let t36543 = t8998 * t7994;
    let t36547 = t525 * t1221;
    (t36531, t36533, t36541, t36543, t36547)
}
