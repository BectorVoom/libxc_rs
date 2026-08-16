//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1569/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1569<F: Float>(t4781: F, t4785: F, t3313: F, t11277: F, t5988: F, t1117: F, t11275: F, t3411: F, t6106: F, t1157: F, t6105: F, t1164: F) -> (F, F, F, F) {
    let t18262 = t4785 * t4781;
    let t18264 = F::cast_from(0.32163958997385070134e2_f64) * t3313 * t18262;
    let t18265 = t5988 * t11277;
    let t18266 = t18265 * t1117;
    let t18268 = F::cast_from(0.51726012919273400301e3_f64) * t11275 * t18266;
    let t18270 = F::cast_from(0.17315859105681463759e2_f64) * t3411 * t6106;
    let t18271 = t6105 * t1157;
    let t18273 = F::cast_from(0.35089341735807877242e1_f64) * t1164 * t18271;
    (t18264, t18268, t18270, t18273)
}
