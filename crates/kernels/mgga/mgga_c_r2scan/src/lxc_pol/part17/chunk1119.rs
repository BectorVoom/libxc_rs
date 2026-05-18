//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1119/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1119<F: Float>(t38153: F, t10868: F, t2147: F, t8066: F, t10856: F, t7470: F, t38166: F, t10708: F, t7262: F, t3281: F, t10848: F, t11760: F, t2207: F) -> (F, F, F, F, F, F, F) {
    let t40238 = F::new(0.57829097596741960692e-3) * t38153;
    let t40241 = t2147 * t10868 * t8066;
    let t40243 = t10856 * t7470;
    let t40248 = F::new(0.84755945902752848174e0) * t38166;
    let t40251 = t10708 * t7262;
    let t40257 = t3281 * t7470;
    let t40260 = t2207 * t11760 * t10848;
    (t40238, t40241, t40243, t40248, t40251, t40257, t40260)
}
