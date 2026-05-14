//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 807/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk807<F: Float>(t1004: F, t3102: F, t3062: F, t3077: F, t1160: F, t180: F, t3101: F, t407: F, t12265: F, t150: F, t1233: F, t3832: F, t1244: F, t3243: F, t3088: F, t3089: F, t930: F) -> (F, F, F, F, F, F, F, F) {
    let t12285 = 0.26341796731742046395e1 * t1004 * t3102;
    let t12286 = t3077 * t3062;
    let t12290 = t1160 * t180 * t3101 * t407;
    let t12295 = t12265 * t150;
    let t12297 = 0.79025390195226139183e1 * t12295 * t1233;
    let t12298 = t1004 * t3832;
    let t12301 = 0.39512695097613069592e1 * t3243 * t1244;
    let t12305 = t3088 * t3089 * t930;
    (t12285, t12286, t12290, t12295, t12297, t12298, t12301, t12305)
}
