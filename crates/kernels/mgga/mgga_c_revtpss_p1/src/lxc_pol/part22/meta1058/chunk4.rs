//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3757/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3757<F: Float>(t1222: F, t17471: F, t20298: F, t20302: F, t1260: F, t57465: F, t21334: F, t1042: F, t1261: F, t1266: F, t12832: F, t17265: F, t17347: F, t21143: F, t21166: F, t21275: F, t3600: F, t3604: F, t3640: F, t3644: F, t5302: F, t5312: F, t59159: F, t65433: F, t68324: F, t68355: F, t70343: F) -> F {
    let t71571 = t1222 * t17471 * t20298;
    let t71582 = t1222 * t17471 * t20302;
    let t71585 = t57465 * t1260;
    let t71590 = t21334 * t1260;
    let t71597 = F::cast_from(0.47637797908966374414e-3_f64) * t1261 * t1042 * t5302 * t65433 + F::cast_from(0.85748036236139473944e-3_f64) * t3600 * t1042 * t70343 * t3604 + t71571 / F::cast_from(54.0_f64) + t1222 * t5312 * t68355 / F::cast_from(108.0_f64) + t1222 * t5312 * t68324 / F::cast_from(36.0_f64) - F::cast_from(0.85748036236139473944e-3_f64) * t12832 * t21166 + t71582 / F::cast_from(162.0_f64) + F::cast_from(0.17149607247227894789e-2_f64) * t59159 - F::cast_from(0.25724410870841842183e-2_f64) * t71585 * t17347 + F::cast_from(0.85748036236139473944e-3_f64) * t21275 * t17265 - F::cast_from(0.28582678745379824648e-3_f64) * t71590 * t1266 - F::cast_from(0.14291339372689912324e-3_f64) * t21143 * t3640 - F::cast_from(0.28582678745379824648e-3_f64) * t21143 * t3644;
    t71597
}
