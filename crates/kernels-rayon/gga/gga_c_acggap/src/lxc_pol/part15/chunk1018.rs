//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1018/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1018(t1449: f64, t30148: f64, t30159: f64, t7586: f64, t1541: f64, t31611: f64, t30219: f64, t8473: f64, t4680: f64, t7426: f64, t8605: f64, t30468: f64, t4916: f64) -> (f64, f64, f64, f64, f64) {
    let t35788 = t30159 * t7586 * t30148 * t1449;
    let t35790 = t31611 * t1541;
    let t35794 = t30219 * t8473;
    let t35797 = t7426 * t4680 * t8605;
    let t35799 = t30468 * t4916;
    (t35788, t35790, t35794, t35797, t35799)
}
