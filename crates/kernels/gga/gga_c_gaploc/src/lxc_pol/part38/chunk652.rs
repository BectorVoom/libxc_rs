//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 652/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk652<F: Float>(t1559: F, t197: F, t1563: F, t202: F, t4526: F, t561: F, t4539: F, t524: F, t123: F, t160: F, t4348: F, t892: F, t10523: F, t1422: F, t544: F, t1564: F, t165: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18535 = t1559 * t197;
    let t18540 = 1.0 / t1563 / t202;
    let t18651 = t561 * t4526;
    let t18658 = t524 * t4539;
    let t19531 = t1559 * t123;
    let t19532 = t19531 * t160;
    let t19933 = t892 * t4348;
    let t20367 = t544 * t10523 * t1422;
    let t20368 = t165 * t1564;
    (t18535, t18540, t18651, t18658, t19531, t19532, t19933, t20367, t20368)
}
