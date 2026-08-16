//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1243/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1243(t120130: f64, t26103: f64, t7467: f64, t26135: f64, t6517: f64, t33211: f64, t6534: f64, t120121: f64, t120123: f64, t120125: f64, t120127: f64, t120129: f64, t31237: f64, t31239: f64, t33152: f64, t33154: f64, t8446: f64) -> f64 {
    let t120131 = 2.0_f64 * t120130;
    let t120132 = t26103 * t7467;
    let t120134 = t6517 * t26135;
    let t120137 = 4.0_f64 * t33211 * t6534;
    let t120138 = t8446 + t33152 + t33154 + t31237 + t31239 + t120121 + t120123 + t120125 + t120127 + t120129 + t120131 + 4.0_f64 * t120132 + 4.0_f64 * t120134 + t120137;
    t120138
}
