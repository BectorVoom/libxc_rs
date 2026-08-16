//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1341/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1341<F: Float>(t17276: F, t1650: F, t3954: F, t4163: F, t4162: F, t4160: F, t1924: F, t3960: F, t11862: F, t5638: F, t1928: F, t4169: F) -> (F, F, F, F, F) {
    let t17277 = F::cast_from(0.14739506172839506172e-2_f64) * t17276;
    let t17279 = t4163 * t1650 * t3954;
    let t17280 = t4162 * t17279;
    let t17281 = t4160 * t17280;
    let t17287 = t1924 * t3960;
    let t17290 = t11862 * t5638;
    let t17292 = t4169 * t1928;
    (t17277, t17281, t17287, t17290, t17292)
}
