//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1693/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1693(t22845: f64, t28073: f64, t1998: f64, t236: f64, t6347: f64, t6926: f64, t6375: f64, t6916: f64, t22761: f64, t6390: f64, t2002: f64, t6378: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28074 = t22845 * t28073;
    let t28077 = t1998 * t236 * t6347;
    let t28078 = t6926 * t28077;
    let t28080 = t6916 * t6375;
    let t28085 = t22761 * t6390;
    let t28088 = t6378 * t2002;
    (t28074, t28077, t28078, t28080, t28085, t28088)
}
