//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2600/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2600(t3577: f64, t44951: f64, t4953: f64, t11677: f64, t15245: f64, t1174: f64, t14753: f64, t3431: f64, t14744: f64, t11651: f64, t15438: f64, t1227: f64, t13969: f64, t15540: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52758 = t3577 * t44951 * t4953;
    let t52766 = t15245 * t11677;
    let t52773 = t1174 * t3431 * t14753;
    let t52776 = t1174 * t3431 * t14744;
    let t52781 = t15438 * t11651;
    let t52792 = t1227 * t13969 * t15540;
    (t52758, t52766, t52773, t52776, t52781, t52792)
}
