//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2003/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2003(t13969: f64, t4988: f64, t1227: f64, t15708: f64, t4723: f64, t11668: f64, t1725: f64, t698: f64, t1174: f64, t1230: f64, t14706: f64, t248: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15743 = t13969 * t4988;
    let t15745 = 5.0_f64 / 10368.0_f64 * t1227 * t15743;
    let t15749 = t4723 * t15708;
    let t15750 = t11668 * t15749;
    let t15753 = t698 * t1725;
    let t15754 = t1174 * t15753;
    let t15761 = t248 * t1230 * t14706;
    (t15743, t15745, t15749, t15750, t15753, t15754, t15761)
}
