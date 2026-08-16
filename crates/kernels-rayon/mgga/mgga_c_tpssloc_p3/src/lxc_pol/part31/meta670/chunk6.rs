//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1995/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1995(t100664: f64, t100705: f64, t100708: f64, t100766: f64, t100788: f64, t101226: f64, t18196: f64, t1877: f64, t2057: f64, t24191: f64, t24339: f64, t2522: f64, t25898: f64, t25901: f64, t25938: f64, t26563: f64, t28795: f64, t29106: f64, t29157: f64, t46341: f64, t6841: f64, t6848: f64, t7656: f64, t7845: f64, t92276: f64, t92319: f64) -> f64 {
    let t102048 = -3.0_f64 / 2.0_f64 * t24191 * t100664 - 3.0_f64 * t24191 * t100705 + 3.0_f64 * t2522 * t7845 * t25938 + 3.0_f64 * t46341 * t29157 - t1877 * t92276 * t7656 - 3.0_f64 * t24191 * t100766 + 6.0_f64 * t26563 * t100708 - 6.0_f64 * t26563 * t100788 + 3.0_f64 * t2522 * t7845 * t25901 - 3.0_f64 * t92319 * t25898 + t1877 * t2057 * t18196 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t29106 * t6841 - t1877 * t101226 * t6848 / 2.0_f64 - t1877 * t24339 * t28795 / 2.0_f64;
    t102048
}
