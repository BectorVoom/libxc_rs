//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1231/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1231(t22144: f64, t32145: f64, t549: f64, t7390: f64, t8756: f64, t6111: f64, t8769: f64, t2365: f64, t24745: f64, t10867: f64, t29021: f64, t29030: f64, t3040: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32875 = 0.2044956050875773316e1_f64 * t22144 * t32145;
    let t32877 = t7390 * t549 * t8756;
    let t32878 = 0.59584149919750711116e-1_f64 * t32877;
    let t32880 = t6111 * t549 * t8769;
    let t32881 = 0.11916829983950142223e0_f64 * t32880;
    let t32883 = t6111 * t2365 * t24745;
    let t32884 = 0.29792074959875355558e-1_f64 * t32883;
    let t32885 = t10867 * t29021;
    let t32886 = 0.10427226235956374445e0_f64 * t32885;
    let t32888 = 0.35750489951850426669e0_f64 * t29030 * t3040;
    (t32875, t32878, t32881, t32884, t32886, t32888)
}
