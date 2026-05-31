//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 732/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk732<F: Float>(t1775: F, t4220: F, t2347: F, t852: F, t2360: F, t14635: F, t14637: F, t14639: F, t14657: F, t14683: F, t14715: F, t14895: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15028 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1775 * t4220;
    let t15042 = t852 * t2347;
    let t15047 = t852 * t2360;
    let t15081 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t14635;
    let t15082 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t14637;
    let t15083 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t14639;
    let t15089 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t14657;
    let t15096 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14683;
    let t15111 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t14715;
    let t15116 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t14895;
    (t15028, t15042, t15047, t15081, t15082, t15083, t15089, t15096, t15111, t15116)
}
