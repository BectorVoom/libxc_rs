//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 425/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk425<F: Float>(t4461: F, t465: F, t479: F, t198: F, t2184: F, t1193: F, t1198: F, t1190: F, t1219: F, t1212: F, t209: F, t1180: F, t1189: F, t1186: F, t1243: F, t195: F) -> (F, F, F, F, F, F, F, F) {
    let t4555 = t465 * t4461;
    let t4556 = t4555 * t479;
    let t4558 = t2184 * t198;
    let t4559 = t1193 * t4558;
    let t4560 = t4559 * t1198;
    let t4562 = t1190 * t1219;
    let t4564 = t1212 * t209;
    let t4569 = t1180 * t1189;
    let t4570 = t4569 * t1186;
    let t4580 = t195 * t1243;
    (t4555, t4556, t4559, t4560, t4562, t4564, t4570, t4580)
}
