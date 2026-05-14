//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 656/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk656<F: Float>(t1212: F, t870: F, t4147: F, t8392: F, t4257: F, t4262: F, t10580: F, t309: F, t312: F, t9570: F, t4142: F, t9577: F, t1882: F, t4252: F, t1225: F, t8232: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15370 = t870 * t1212;
    let t15376 = 2.0 / 27.0 * t8392 * t4147;
    let t15382 = 2.0 / 27.0 * t8392 * t4257;
    let t15384 = 2.0 / 27.0 * t8392 * t4262;
    let t15385 = t10580 * t309;
    let t15386 = t312 * t9570;
    let t15400 = 4.0 / 81.0 * t8392 * t4142;
    let t15402 = t312 * t9577;
    let t15419 = 2.0 / 9.0 * t1882 * t4252;
    let t15420 = t8232 * t1225;
    (t15370, t15376, t15382, t15384, t15385, t15386, t15400, t15402, t15419, t15420)
}
