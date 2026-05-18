//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1009/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1009<F: Float>(t19621: F, t3699: F, t15254: F, t3690: F, t15294: F, t1091: F, t4167: F, t10703: F, t5376: F, t681: F, t89: F, t14961: F, t17749: F) -> (F, F, F, F, F) {
    let t19622 = t3699 * t19621;
    let t19623 = t15254 * t19622;
    let t19626 = t3690 * t19621;
    let t19627 = t15294 * t19626;
    let t19630 = t1091 * t4167;
    let t19631 = t10703 * t19630;
    let t19635 = t89 * t681 * t5376;
    let t19640 = t14961 * t17749;
    (t19623, t19627, t19631, t19635, t19640)
}
