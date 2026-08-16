//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 736/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk736<F: Float>(t1882: F, t4276: F, t4280: F, t2681: F, t309: F, t1212: F, t870: F, t4147: F, t8392: F, t4257: F, t4262: F, t10580: F) -> (F, F, F, F, F, F, F, F) {
    let t15334 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t4276;
    let t15336 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t4280;
    let t15369 = t2681 * t309;
    let t15370 = t870 * t1212;
    let t15376 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8392 * t4147;
    let t15382 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8392 * t4257;
    let t15384 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8392 * t4262;
    let t15385 = t10580 * t309;
    (t15334, t15336, t15369, t15370, t15376, t15382, t15384, t15385)
}
