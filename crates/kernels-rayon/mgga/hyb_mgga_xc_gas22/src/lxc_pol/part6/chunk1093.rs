//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1093/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1093(t10658: f64, t809: f64, t6562: f64, t3419: f64, t3435: f64, t4180: f64, t6640: f64, t3444: f64, t2289: f64, t4193: f64, t849: f64, t260: f64, t4175: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10659 = t10658 * t809;
    let t10661 = 0.51726012919273400301e3_f64 * t6562 * t10659;
    let t10662 = t3435 * t3419;
    let t10667 = t6640 * t4180;
    let t10668 = t10667 * t3444;
    let t10671 = t2289 * t4193;
    let t10672 = t10671 * t849;
    let t10679 = t260 * t4175;
    (t10659, t10661, t10662, t10667, t10668, t10671, t10672, t10679)
}
