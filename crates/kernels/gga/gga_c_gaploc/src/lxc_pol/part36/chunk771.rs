//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 771/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk771<F: Float>(t1: F, t1559: F, t544: F, t986: F, t10241: F, t1359: F, t12380: F, t455: F, t145: F, t459: F, t12385: F, t2281: F) -> (F, F, F, F, F, F) {
    let t35204 = t544 * t1559 * t986 * t1;
    let t35215 = t1359 * t10241;
    let t35216 = t544 * t35215;
    let t39622 = t12380 * t455;
    let t39624 = t39622 * t145 * t459;
    let t39626 = t2281 * t12385;
    (t35204, t35215, t35216, t39622, t39624, t39626)
}
