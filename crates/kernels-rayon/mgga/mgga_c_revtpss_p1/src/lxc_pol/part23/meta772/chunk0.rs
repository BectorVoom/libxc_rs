//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2575/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2575(t17395: f64, t3746: f64, t12268: f64, t29054: f64, t12898: f64, t1786: f64, t17202: f64, t372: f64, t44546: f64, t5340: f64, t5342: f64, t11772: f64, t17394: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57571 = t3746 * t17395;
    let t57606 = t29054 * t12268;
    let t57615 = t1786 * t12898;
    let t57621 = t372 * t17202;
    let t57635 = t5340 * t44546 * t5342;
    let t57636 = 0.28582678745379824648e-3_f64 * t57635;
    let t57659 = t17394 * t11772;
    (t57571, t57606, t57615, t57621, t57636, t57659)
}
