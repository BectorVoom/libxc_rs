//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1158/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1158<F: Float>(t6568: F, t8045: F, t2798: F, t7058: F, t6556: F, t8060: F, t2497: F, t8042: F, t8057: F, t10301: F, t4342: F, t1016: F, t1382: F) -> (F, F, F, F, F, F, F) {
    let t31465 = F::cast_from(4.0_f64) * t8045 * t6568;
    let t31470 = t2798 * t7058;
    let t31472 = F::cast_from(2.0_f64) * t6556 * t8060;
    let t31474 = F::cast_from(2.0_f64) * t8042 * t2497;
    let t31476 = F::cast_from(4.0_f64) * t6556 * t8057;
    let t31480 = F::cast_from(4.0_f64) * t4342 * t10301;
    let t31483 = F::cast_from(2.0_f64) * t1382 * t1016 * t7058;
    (t31465, t31470, t31472, t31474, t31476, t31480, t31483)
}
