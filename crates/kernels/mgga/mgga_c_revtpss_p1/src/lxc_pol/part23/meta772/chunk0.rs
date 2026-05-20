//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2575/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2575<F: Float>(t17395: F, t3746: F, t12268: F, t29054: F, t12898: F, t1786: F, t17202: F, t372: F, t44546: F, t5340: F, t5342: F, t11772: F, t17394: F) -> (F, F, F, F, F, F) {
    let t57571 = t3746 * t17395;
    let t57606 = t29054 * t12268;
    let t57615 = t1786 * t12898;
    let t57621 = t372 * t17202;
    let t57635 = t5340 * t44546 * t5342;
    let t57636 = F::cast_from(0.28582678745379824648e-3_f64) * t57635;
    let t57659 = t17394 * t11772;
    (t57571, t57606, t57615, t57621, t57636, t57659)
}
