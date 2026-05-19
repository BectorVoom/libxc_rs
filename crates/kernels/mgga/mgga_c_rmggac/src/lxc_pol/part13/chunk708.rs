//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 708/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk708<F: Float>(t2231: F, t534: F, t72: F, t530: F, t8188: F, t2474: F, t302: F, t8328: F, t8331: F, t8334: F, t8350: F, t8356: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9658 = t534 * t2231;
    let t9659 = t72 * t9658;
    let t9675 = t530 * t8188;
    let t9677 = t302 * t2474;
    let t9678 = t72 * t9677;
    let t10244 = F::cast_from(0.3842256877732895568e-2_f64) * t8328;
    let t10245 = F::cast_from(0.162600798888400151e-2_f64) * t8331;
    let t10246 = F::cast_from(0.162600798888400151e-2_f64) * t8334;
    let t10250 = F::cast_from(0.60975299583150056624e-3_f64) * t8350;
    let t10251 = F::cast_from(0.60975299583150056624e-3_f64) * t8356;
    (t9658, t9659, t9675, t9677, t9678, t10244, t10245, t10246, t10250, t10251)
}
