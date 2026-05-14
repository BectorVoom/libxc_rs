//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 653/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk653<F: Float>(t2360: F, t852: F, t14635: F, t14637: F, t14639: F, t14657: F, t14683: F, t14715: F, t14895: F, t14902: F, t1240: F, t2842: F, t4239: F, t870: F, t1250: F, t8232: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15047 = t852 * t2360;
    let t15081 = 2.0 / 27.0 * t14635;
    let t15082 = 4.0 / 27.0 * t14637;
    let t15083 = 4.0 / 81.0 * t14639;
    let t15089 = 2.0 / 27.0 * t14657;
    let t15096 = 4.0 / 9.0 * t14683;
    let t15111 = 4.0 / 81.0 * t14715;
    let t15116 = 4.0 / 27.0 * t14895;
    let t15118 = 2.0 / 9.0 * t14902;
    let t15128 = t1240 * t2842;
    let t15133 = t4239 * t870;
    let t15147 = t8232 * t1250;
    (t15047, t15081, t15082, t15083, t15089, t15096, t15111, t15116, t15118, t15128, t15133, t15147)
}
