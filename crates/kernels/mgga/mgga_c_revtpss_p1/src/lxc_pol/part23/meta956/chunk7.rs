//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3198/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3198<F: Float>(t17351: F, t17353: F, t20766: F, t20767: F, t20929: F, t3611: F, t44510: F, t44521: F, t44829: F, t5406: F, t57660: F, t69832: F, t71061: F, t71373: F, t71377: F, t71400: F, t71435: F, t71447: F, t71460: F, t83760: F) -> F {
    let t84020 = -t71373 / F::new(72.0) - t71377 / F::new(48.0) - F::cast_from(0.28582678745379824648e-3_f64) * t71400 - F::cast_from(0.63517063878621832551e-4_f64) * t44829 - F::cast_from(0.45732285992607719436e-2_f64) * t57660 * t20929 - F::cast_from(0.17149607247227894789e-2_f64) * t71447 * t20767 + F::cast_from(0.85748036236139473944e-3_f64) * t17351 * t17353 * t3611 * t83760 + F::cast_from(0.95275595817932748825e-3_f64) * t71435 - F::cast_from(0.85748036236139473944e-3_f64) * t44521 * t69832 * t5406 + F::cast_from(0.17149607247227894789e-2_f64) * t44510 * t71061 * t20766 + t71460 / F::new(54.0);
    t84020
}
