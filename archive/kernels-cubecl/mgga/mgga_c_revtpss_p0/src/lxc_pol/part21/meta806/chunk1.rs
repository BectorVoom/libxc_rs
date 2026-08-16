//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2935/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2935<F: Float>(t11710: F, t15964: F, t3091: F, t11683: F, t11774: F, t12131: F, t15689: F, t15691: F, t15693: F, t15696: F, t15963: F, t42170: F, t42172: F, t42176: F, t42190: F, t53402: F, t53407: F, t53413: F, t53416: F) -> F {
    let t53422 = t3091 * t11710 * t15964;
    let t53425 = F::cast_from(0.85748036236139473944e-3_f64) * t11774 * t15696 * t11683 + F::cast_from(0.45732285992607719436e-2_f64) * t53402 * t15693 - F::cast_from(0.57165357490759649295e-3_f64) * t53407 + F::cast_from(0.85748036236139473944e-3_f64) * t15689 * t15691 * t12131 * t15963 - F::cast_from(0.42874018118069736972e-3_f64) * t53413 + F::cast_from(0.85748036236139473944e-3_f64) * t53416 - F::cast_from(0.85748036236139473944e-3_f64) * t42170 - F::cast_from(0.45732285992607719436e-2_f64) * t42172 - F::cast_from(0.28582678745379824648e-3_f64) * t42176 - F::cast_from(0.57165357490759649295e-3_f64) * t53422 - F::cast_from(0.57165357490759649295e-3_f64) * t42190;
    t53425
}
