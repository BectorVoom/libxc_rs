//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2587/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2587<F: Float>(t10008: F, t1358: F, t212: F, t689: F, t1359: F, t39501: F, t10115: F, t555: F, t1445: F, t10165: F, t9664: F, t1427: F, t1444: F, t22: F, t9647: F) -> (F, F, F, F, F, F) {
    let t47558 = t689 * t212 * t10008 * t1358;
    let t47561 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t1359;
    let t47567 = t10115 * t555;
    let t47568 = t47567 * t1445;
    let t47570 = t10165 * t9664;
    let t47574 = t9647 * t1427 * t22 * t1444;
    (t47558, t47561, t47567, t47568, t47570, t47574)
}
