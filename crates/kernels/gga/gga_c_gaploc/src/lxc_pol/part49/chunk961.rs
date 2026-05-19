//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 961/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk961<F: Float>(t1358: F, t7888: F, t9199: F, t2321: F, t35215: F, t9074: F, t3394: F, t4385: F, t9078: F, t10256: F, t30204: F, t6525: F) -> (F, F, F, F) {
    let t42537 = F::cast_from(0.94850022118920498663e-2_f64) * t1358 * t7888 * t9199;
    let t42539 = t9074 * t35215 * t2321;
    let t42540 = F::cast_from(0.23712505529730124666e-2_f64) * t42539;
    let t42544 = F::cast_from(0.22131671827748116354e-1_f64) * t1358 * t9078 * t3394 * t4385;
    let t42546 = t6525 * t30204 * t10256;
    (t42537, t42540, t42544, t42546)
}
