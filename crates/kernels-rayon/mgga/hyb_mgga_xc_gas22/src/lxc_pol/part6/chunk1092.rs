//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1092/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1092(t4143: f64, t809: f64, t6579: f64, t4140: f64, t2188: f64, t2236: f64, t4139: f64, t2234: f64, t3352: f64, t3356: f64, t4113: f64, t6564: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10645 = t4143 * t809;
    let t10647 = 0.96491876992155210402e2_f64 * t6579 * t10645;
    let t10648 = t4140 * t809;
    let t10650 = 2.0_f64 * t2188 * t10648;
    let t10651 = t4139 * t2236;
    let t10652 = t10651 * t809;
    let t10654 = 0.16081979498692535067e2_f64 * t2234 * t10652;
    let t10655 = t3356 * t3352;
    let t10657 = 0.32163958997385070134e2_f64 * t2234 * t10655;
    let t10658 = t4113 * t6564;
    (t10645, t10647, t10648, t10650, t10651, t10652, t10654, t10655, t10657, t10658)
}
