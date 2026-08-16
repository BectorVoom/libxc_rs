//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 437/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk437(t171: f64, t1871: f64, t584: f64, t406: f64, t661: f64, t1399: f64, t1732: f64, t1734: f64, t1738: f64, t1740: f64, t189: f64, t183: f64) -> (f64, f64, f64, f64, f64) {
    let t1874 = 0.571528e-1_f64 * t584 * t171 * t1871;
    let t1875 = t406 * t661;
    let t1882 = 0.126595e1_f64 * t1732 - 0.33758666666666666667e1_f64 * t1734 - 0.13651666666666666667e0_f64 * t1738 + 0.27303333333333333333e0_f64 * t1740 + 0.10954222222222222222e0_f64 * t1399;
    let t1883 = t1882 * t189;
    let t1885 = 1.0_f64 * t183 * t1883;
    (t1874, t1875, t1882, t1883, t1885)
}
