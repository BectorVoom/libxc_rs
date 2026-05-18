//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1001/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1001<F: Float>(t15195: F, t4266: F, t1240: F, t2766: F, t4141: F, t5410: F, t8392: F, t1212: F, t2842: F, t4181: F, t15460: F, t5415: F) -> (F, F, F, F, F) {
    let t19497 = t15195 * t4266;
    let t19500 = t2766 * t1240;
    let t19501 = t19500 * t4141;
    let t19504 = t8392 * t5410;
    let t19506 = t2842 * t1212;
    let t19507 = t19506 * t4181;
    let t19508 = t15460 * t19507;
    let t19511 = t8392 * t5415;
    (t19497, t19501, t19504, t19508, t19511)
}
