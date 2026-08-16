//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1071/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1071<F: Float>(t1928: F, t610: F, t990: F, t4426: F, t6141: F, t25: F, t494: F, t6178: F, t1599: F, t1369: F, t2470: F, t6164: F) -> (F, F, F, F) {
    let t18192 = t610 * t1928 * t990;
    let t18205 = t6141 * t4426 / F::cast_from(324.0_f64);
    let t18210 = t25 * t494;
    let t18211 = t18210 * t6178;
    let t18213 = t1599 * t18211 / F::cast_from(144.0_f64);
    let t18221 = t2470 * t1369;
    let t18222 = t18221 * t6164;
    (t18192, t18205, t18213, t18222)
}
