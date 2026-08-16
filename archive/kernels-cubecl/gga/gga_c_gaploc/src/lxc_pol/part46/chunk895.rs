//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 895/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk895<F: Float>(t42522: F, t1365: F, t31558: F, t6525: F, t12963: F, t1358: F, t2299: F, t488: F, t7888: F, t9199: F, t2321: F, t35215: F, t9074: F) -> (F, F, F, F, F) {
    let t42523 = F::cast_from(4.0_f64) * t42522;
    let t42529 = t6525 * t1365 * t31558;
    let t42533 = t1358 * t2299 * t12963 * t488;
    let t42537 = F::cast_from(0.94850022118920498663e-2_f64) * t1358 * t7888 * t9199;
    let t42539 = t9074 * t35215 * t2321;
    (t42523, t42529, t42533, t42537, t42539)
}
