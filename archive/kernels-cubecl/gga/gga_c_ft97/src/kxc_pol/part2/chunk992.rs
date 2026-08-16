//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 992/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk992<F: Float>(t15377: F, t4139: F, t4257: F, t8392: F, t4262: F, t10580: F, t309: F, t312: F, t9570: F, t13863: F, t2413: F, t4145: F) -> (F, F, F, F, F) {
    let t15378 = t4139 * t15377;
    let t15382 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8392 * t4257;
    let t15384 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8392 * t4262;
    let t15385 = t10580 * t309;
    let t15386 = t312 * t9570;
    let t15387 = t15386 * t13863;
    let t15388 = t15385 * t15387;
    let t15391 = t4145 * t2413;
    (t15378, t15382, t15384, t15388, t15391)
}
