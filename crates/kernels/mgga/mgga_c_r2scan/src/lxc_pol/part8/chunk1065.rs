//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1065/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1065<F: Float>(t40: F, t56: F, t60: F, t88: F, t406: F, t4694: F, t1751: F, t4962: F, t378: F, t5037: F, t735: F, t468: F, t4715: F, t5002: F, t1398: F, t1524: F) -> (F, F, F, F, F, F, F, F) {
    let t18852 = t40 * t56;
    let t18855 = 24.0 * t18852 * t60 * t88;
    let t18856 = t406 * t4694;
    let t18865 = t1751 * t4962;
    let t18869 = 0.38527786510141256862e1 * t735 * t378 * t5037;
    let t18872 = 0.67471172535210825684e-1 * t735 * t4715 * t468;
    let t18875 = 0.21687162600603479684e-1 * t735 * t378 * t5002;
    let t18878 = 0.86748650402413918736e-1 * t735 * t1398 * t1524;
    (t18852, t18855, t18856, t18865, t18869, t18872, t18875, t18878)
}
