//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1183/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1183(t11717: f64, t26278: f64, t10760: f64, t29700: f64, t6085: f64, t11693: f64, t8198: f64, t10856: f64, t9319: f64, t12455: f64, t3336: f64, t5103: f64) -> (f64, f64, f64, f64, f64) {
    let t43480 = t26278 * t11717;
    let t43483 = t6085 * t10760 * t29700;
    let t43488 = t8198 * t11693;
    let t43490 = t10856 * t9319;
    let t43495 = t5103 * t3336 * t12455;
    (t43480, t43483, t43488, t43490, t43495)
}
