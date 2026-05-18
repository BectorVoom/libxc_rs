//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 859/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk859<F: Float>(t17220: F, t17225: F, t17246: F, t17353: F, t605: F, t144: F, t1882: F, t4819: F, t4815: F, t3478: F, t925: F, t9144: F) -> (F, F, F, F, F) {
    let t17355 = t17220 + t17225 + t17246 + t17353;
    let t17356 = t605 * t17355;
    let t17357 = t144 * t17356;
    let t17360 = t1882 * t4819;
    let t17362 = t1882 * t4815;
    let t17365 = t925 * t3478;
    let t17366 = t9144 * t17365;
    (t17356, t17357, t17360, t17362, t17366)
}
