//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 837/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk837<F: Float>(t16503: F, t35039: F, t352: F, t38422: F, t38649: F, t1652: F, t7778: F, t739: F, t1550: F, t2060: F, t27124: F, t8542: F, t9128: F) -> (F, F, F, F, F) {
    let t38663 = t16503 * t35039 * t38422 * t38649 * t352;
    let t38674 = t7778 * t1652;
    let t38675 = t739 * t38674;
    let t38676 = F::cast_from(0.79828278012425390426e-1_f64) * t38675;
    let t38678 = t1550 * t2060 * t27124;
    let t38680 = t9128 * t8542;
    (t38663, t38674, t38676, t38678, t38680)
}
