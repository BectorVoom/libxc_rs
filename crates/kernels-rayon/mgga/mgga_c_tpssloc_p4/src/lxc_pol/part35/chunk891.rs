//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 891/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk891(t1667: f64, t2403: f64, t1657: f64, t3263: f64, t3312: f64, t1720: f64, t3030: f64, t3609: f64, t1687: f64, t3400: f64, t3375: f64, t1675: f64, t3356: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14766 = t2403 * t1667;
    let t14838 = t1657 * t3263;
    let t14850 = t1657 * t3312;
    let t15026 = t1720 * t3030;
    let t15027 = t15026 * t3609;
    let t15126 = t1687 * t3400;
    let t15136 = t1687 * t3375;
    let t15146 = t1675 * t3356;
    (t14766, t14838, t14850, t15026, t15027, t15126, t15136, t15146)
}
