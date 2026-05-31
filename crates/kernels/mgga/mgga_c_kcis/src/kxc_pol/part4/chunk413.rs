//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 413/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk413<F: Float>(t1305: F, t1328: F, t1309: F, t1320: F, t1325: F, t1332: F) -> (F, F, F) {
    let t1566 = F::cast_from(0.516475e0_f64) * t1305;
    let t1569 = F::cast_from(0.104195e0_f64) * t1328;
    let t1571 = F::cast_from(0.3529725e1_f64) * t1320 - t1566 - F::cast_from(0.516475e0_f64) * t1309 + F::cast_from(0.6311625e0_f64) * t1325 - t1569 - F::cast_from(0.104195e0_f64) * t1332;
    (t1566, t1569, t1571)
}
