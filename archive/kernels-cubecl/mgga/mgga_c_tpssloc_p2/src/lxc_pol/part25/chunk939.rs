//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 939/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk939<F: Float>(t2663: F, t3814: F, t3681: F, t67: F, t758: F, t1294: F, t9905: F, t9892: F, t3826: F, t588: F, t3684: F, t9467: F) -> (F, F, F, F, F, F) {
    let t12097 = t3814 * t2663;
    let t12098 = F::cast_from(0.73245789224026180216e-3_f64) * t12097;
    let t12099 = t3681 * t67;
    let t12100 = t12099 * t758;
    let t12101 = F::cast_from(0.54934341918019635162e-3_f64) * t12100;
    let t12103 = F::cast_from(0.35089341735807877242e1_f64) * t1294 * t9905;
    let t12105 = F::cast_from(0.51947577317044391277e2_f64) * t1294 * t9892;
    let t12106 = t588 * t3826;
    let t12107 = F::cast_from(24.0_f64) * t12106;
    let t12109 = F::cast_from(0.21687162600603479684e-1_f64) * t3684 * t9467;
    (t12098, t12101, t12103, t12105, t12107, t12109)
}
