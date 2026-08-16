//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2599/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2599(t1734: f64, t3507: f64, t11721: f64, t11786: f64, t5005: f64, t15730: f64, t3536: f64, t15594: f64, t3523: f64, t1174: f64, t14726: f64, t44562: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52696 = t1734 * t3507;
    let t52704 = t1734 * t11721;
    let t52725 = t5005 * t11786;
    let t52731 = t3536 * t15730;
    let t52733 = t15594 * t3523;
    let t52751 = t1174 * t44562 * t14726;
    (t52696, t52704, t52725, t52731, t52733, t52751)
}
