//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 890/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk890<F: Float>(t38530: F, t9153: F, t2281: F, t34975: F, t35039: F, t8455: F, t14237: F, t16503: F, t9157: F, t38523: F, t9163: F, t34962: F, t9151: F) -> (F, F, F, F, F) {
    let t44755 = t38530 * t9153;
    let t44759 = t34975 * t35039 * t2281 * t8455;
    let t44763 = t16503 * t14237 * t2281 * t9157;
    let t44767 = t16503 * t35039 * t38523 * t9163;
    let t44771 = t16503 * t34962 * t2281 * t9151;
    (t44755, t44759, t44763, t44767, t44771)
}
