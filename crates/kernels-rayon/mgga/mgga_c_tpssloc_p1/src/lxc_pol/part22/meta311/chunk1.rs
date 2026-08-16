//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1486/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1486(t11352: f64, t1682: f64, t14722: f64, t14704: f64, t1675: f64, t3331: f64, t3403: f64, t4857: f64, t11285: f64, t1694: f64, t15026: f64, t3623: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15171 = t1682 * t11352;
    let t15194 = 0.2283111111111111111e-1_f64 * t14722;
    let t15195 = 0.11415555555555555555e-1_f64 * t14704;
    let t15207 = t1675 * t3331;
    let t15218 = t4857 * t3403;
    let t15225 = t1694 * t11285;
    let t15245 = t15026 * t3623;
    (t15171, t15194, t15195, t15207, t15218, t15225, t15245)
}
