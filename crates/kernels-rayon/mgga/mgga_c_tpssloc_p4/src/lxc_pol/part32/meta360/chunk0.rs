//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1410/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1410(t1147: f64, t4832: f64, t1687: f64, t3400: f64, t3375: f64, t1128: f64, t4794: f64, t1675: f64, t3356: f64, t14722: f64, t14704: f64, t3331: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15121 = t4832 * t1147;
    let t15126 = t1687 * t3400;
    let t15136 = t1687 * t3375;
    let t15141 = t4794 * t1128;
    let t15146 = t1675 * t3356;
    let t15194 = 0.2283111111111111111e-1_f64 * t14722;
    let t15195 = 0.11415555555555555555e-1_f64 * t14704;
    let t15207 = t1675 * t3331;
    (t15121, t15126, t15136, t15141, t15146, t15194, t15195, t15207)
}
