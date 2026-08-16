//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 904/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk904(t8561: f64, t9080: f64, t2715: f64, t8549: f64, t8548: f64, t2724: f64, t940: f64, t2813: f64, t375: f64, t1071: f64, t2997: f64, t3000: f64, t433: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9081 = t9080 * t8561;
    let t9093 = t8549 * t2715;
    let t9094 = t8548 * t9093;
    let t9095 = t9080 * t2724;
    let t9116 = t8549 * t940;
    let t9117 = t8548 * t9116;
    let t9133 = 1.0_f64 / t2813 / t375;
    let t9172 = 1.0_f64 / t2997 / t1071;
    let t9176 = 1.0_f64 / t3000 / t433;
    (t9081, t9094, t9095, t9117, t9133, t9172, t9176)
}
