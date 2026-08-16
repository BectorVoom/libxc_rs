//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1701/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1701<F: Float>(t6244: F, t6305: F, t1011: F, t1063: F, t1066: F, t11632: F, t11774: F, t1469: F, t15689: F, t15696: F, t16012: F, t16226: F, t23907: F, t247: F, t3117: F, t3155: F, t42472: F, t42621: F, t43050: F, t4893: F, t4915: F, t6266: F, t6267: F, t66777: F, t67052: F, t79219: F, t79233: F, t79253: F, t79450: F, t88087: F, t88095: F, t88102: F, t88106: F, t88116: F, t88794: F) -> (F, F) {
    let t89084 = t6244 * t6305;
    let t89094 = F::cast_from(7.0_f64) / F::cast_from(108.0_f64) * t1011 * t16012 * t88116 + t1011 * t4915 * t88087 / F::cast_from(8.0_f64) - t1011 * t4915 * t88095 / F::cast_from(36.0_f64) - F::cast_from(0.17149607247227894789e-2_f64) * t15689 * t66777 * t4893 * t6266 - F::cast_from(0.17149607247227894789e-2_f64) * t11774 * t67052 * t6267 - F::cast_from(0.17149607247227894789e-2_f64) * t11774 * t15696 * t23907 + F::cast_from(0.34299214494455789578e-2_f64) * t16226 * t66777 * t3155 * t79450 * t1469 + F::cast_from(0.22866142996303859718e-2_f64) * t79219 + F::cast_from(0.14291339372689912324e-3_f64) * t1063 * t247 * t1066 * t88106 + F::cast_from(0.23289590088828005269e-2_f64) * t1063 * t247 * t42472 * t88102 - F::cast_from(0.17149607247227894789e-2_f64) * t79233 + F::cast_from(0.51448821741683684368e-2_f64) * t43050 * t3117 * t89084 * t3155 - F::cast_from(0.17149607247227894789e-2_f64) * t79253 - F::cast_from(0.51448821741683684368e-2_f64) * t42621 * t3117 * t88794 * t11632;
    (t89084, t89094)
}
