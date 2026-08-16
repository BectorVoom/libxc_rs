//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1221/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1221(t120112: f64, t510: f64, t19456: f64, t8326: f64, t26114: f64, t26117: f64, t31717: f64, t7467: f64, t26135: f64, t8601: f64, t12725: f64, t33211: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t120114 = 2.0_f64 * t120112 * t510;
    let t120120 = t19456 * t8326;
    let t120121 = 2.0_f64 * t120120;
    let t120122 = t26114 * t8326;
    let t120123 = 2.0_f64 * t120122;
    let t120124 = t26117 * t8326;
    let t120125 = 2.0_f64 * t120124;
    let t120127 = 4.0_f64 * t31717 * t7467;
    let t120129 = 4.0_f64 * t8601 * t26135;
    let t120130 = t12725 * t8326;
    let t120131 = 2.0_f64 * t120130;
    let t120137 = 4.0_f64 * t33211 * t6534;
    (t120114, t120121, t120123, t120125, t120127, t120129, t120131, t120137)
}
