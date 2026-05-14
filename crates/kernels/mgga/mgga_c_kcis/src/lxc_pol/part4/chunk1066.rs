//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1066/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1066<F: Float>(t3577: F, t5233: F, t1219: F, t3569: F, t5237: F, t10865: F, t1830: F, t3551: F, t5250: F, t969: F, t1835: F, t3025: F, t3006: F, t5253: F, t3034: F, t4758: F) -> (F, F, F, F, F, F, F) {
    let t15327 = t5233 * t3577;
    let t15328 = t15327 * t1219;
    let t15331 = t5237 * t3569;
    let t15334 = t1830 * t10865;
    let t15335 = t15334 * t3551;
    let t15342 = t5250 * t969;
    let t15345 = t1835 * t3025;
    let t15348 = t5253 * t3006;
    let t15351 = t4758 * t3034;
    (t15328, t15331, t15335, t15342, t15345, t15348, t15351)
}
