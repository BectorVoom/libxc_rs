//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 983/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk983<F: Float>(t74948: F, t14710: F, t2868: F, t739: F, t7799: F, t9530: F, t74953: F, t74957: F, t3351: F, t498: F, t515: F, t7248: F, t9523: F) -> (F, F, F, F, F, F) {
    let t77281 = F::cast_from(0.2553875993597870364e-4_f64) * t74948;
    let t77282 = t2868 * t14710;
    let t77283 = F::cast_from(0.2993560425465952141e-1_f64) * t77282;
    let t77286 = F::cast_from(0.11974241701863808564e0_f64) * t739 * t9530 * t7799;
    let t77287 = F::cast_from(0.2553875993597870364e-4_f64) * t74953;
    let t77288 = F::cast_from(0.7661627980793611092e-4_f64) * t74957;
    let t77292 = t3351 * t7248 * t515 * t9523 * t498;
    (t77281, t77283, t77286, t77287, t77288, t77292)
}
