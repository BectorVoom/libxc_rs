//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1067/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1067(t6376: f64, t699: f64, t37200: f64, t38570: f64, t38608: f64, t38610: f64, t42666: f64, t42685: f64, t42693: f64, t42696: f64, t42697: f64, t42698: f64, t42702: f64, t44866: f64, t44874: f64, t44878: f64, t44882: f64, t44886: f64, t44888: f64, t884: f64) -> (f64, f64) {
    let t48217 = t699 * t6376;
    let t48225 = -t42666 - 0.1454648621559751559e0_f64 * t38570 - 0.638468998399467591e-4_f64 * t44866 - 0.60975299583150056624e-3_f64 * t38608 + 0.60975299583150056624e-3_f64 * t38610 + 0.59871208509319042821e-1_f64 * t884 * t48217 + t42685 + t42693 - t42696 + t42697 + t42698 - t37200 - 0.5107751987195740728e-4_f64 * t44874 + 0.15323255961587222184e-3_f64 * t44878 - 0.20431007948782962912e-3_f64 * t44882 + 0.24829349937757072983e-4_f64 * t44886 - 0.39726959900411316773e-4_f64 * t44888 - t42702;
    (t48217, t48225)
}
