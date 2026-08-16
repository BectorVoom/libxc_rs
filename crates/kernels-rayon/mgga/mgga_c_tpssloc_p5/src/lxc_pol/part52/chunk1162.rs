//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1162/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1162(t6999: f64, t8489: f64, t1983: f64, t3701: f64, t6995: f64, t2019: f64, t6880: f64, t8450: f64, t7000: f64, t12461: f64, t1388: f64, t8493: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31033 = t8489 * t6999;
    let t31034 = t1983 * t31033;
    let t31035 = t3701 * t6995;
    let t31036 = t2019 * t31035;
    let t31038 = 2.0_f64 * t1983 * t31036;
    let t31039 = t8450 * t6880;
    let t31041 = t8450 * t7000;
    let t31043 = t12461 * t1388;
    let t31044 = t8493 * t31043;
    (t31033, t31034, t31036, t31038, t31039, t31041, t31044)
}
