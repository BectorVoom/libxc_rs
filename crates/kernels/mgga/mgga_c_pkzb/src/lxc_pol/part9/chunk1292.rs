//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1292/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1292<F: Float>(t1184: F, t6199: F, t2234: F, t6201: F, t851: F, t22233: F, t18427: F, t18430: F, t18433: F, t18596: F, t22230: F, t22236: F, t22262: F, t378: F) -> (F, F) {
    let t22684 = t6199 * t1184;
    let t22688 = F::cast_from(0.1551780387578202009e4_f64) * t22684 * t6201 * t2234 * t851;
    let t22693 = F::cast_from(0.37083333333333333334e-1_f64) * t22233;
    let t22697 = (t18596 - F::cast_from(0.86527777777777777777e-1_f64) * t18427 + F::cast_from(0.37083333333333333333e-1_f64) * t18430 - F::cast_from(0.92708333333333333333e-2_f64) * t18433 - F::cast_from(0.28842592592592592592e-1_f64) * t22230 + t22693 - F::cast_from(0.278125e-1_f64) * t22236 + F::cast_from(0.278125e-1_f64) * t22262) * t378;
    (t22688, t22697)
}
