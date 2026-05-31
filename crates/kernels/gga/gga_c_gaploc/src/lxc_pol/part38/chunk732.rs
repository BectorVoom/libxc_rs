//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 732/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk732<F: Float>(t10530: F, t1434: F, t584: F, t1559: F, t197: F, t1563: F, t202: F, t4526: F, t561: F, t4539: F, t524: F, t123: F) -> (F, F, F, F, F, F) {
    let t18372 = t584 * t10530 * t1434;
    let t18535 = t1559 * t197;
    let t18540 = F::cast_from(1.0_f64) / t1563 / t202;
    let t18651 = t561 * t4526;
    let t18658 = t524 * t4539;
    let t19531 = t1559 * t123;
    (t18372, t18535, t18540, t18651, t18658, t19531)
}
