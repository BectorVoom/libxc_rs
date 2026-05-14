//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1187/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1187<F: Float>(t10052: F, t10157: F, t107819: F, t107832: F, t107835: F, t1091: F, t109713: F, t1403: F, t193: F, t2354: F, t24191: F, t24231: F, t27943: F, t27953: F, t27991: F, t28010: F, t30904: F, t30909: F, t30933: F, t3746: F, t3837: F, t3972: F, t6002: F, t6745: F, t684: F, t6930: F, t766: F, t96361: F) -> (F,) {
    let t121769 = -2.0 / 3.0 * t1403 * t193 * t24191 * t30904 - 2.0 / 81.0 * t96361 - t6002 * t2354 * t109713 * t1091 / 9.0 + 2.0 / 9.0 * t28010 * t2354 * t27991 * t3746 + t107819 - 24.0 * t10052 * t30933 * t766 + t6002 * t24231 * t30909 * t684 / 9.0 + t6745 * t27943 / 3.0 + t107832 - t107835 + 2.0 * t6002 * t10157 * t27991 * t3837 - 24.0 * t10052 * t6930 * t3972 - 2.0 / 3.0 * t6745 * t27953;
    (t121769,)
}
