//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 744/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk744<F: Float>(t157: F, t406: F, t556: F, t7932: F, t309: F, t525: F, t7963: F, t609: F, t939: F) -> (F, F, F, F, F, F) {
    let t9025 = t556 * t406 * t157;
    let t9026 = t7932 * t9025;
    let t9029 = t525 * t309;
    let t9030 = t7932 * t9029;
    let t9031 = t7963 * t9030;
    let t9033 = t939 * t609;
    (t9025, t9026, t9029, t9030, t9031, t9033)
}
