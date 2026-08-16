//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 932/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk932(t9009: f64, t9011: f64, t9014: f64, t9017: f64, t9021: f64, t9024: f64, t9027: f64, t9032: f64, t9034: f64, t9036: f64, t9038: f64, t9042: f64, t9044: f64) -> f64 {
    let t10679 = -0.2471588561924985691e-3_f64 * t9009 - 0.36652500116630512966e-6_f64 * t9011 - 0.55603792169291016668e-2_f64 * t9014 + 0.15176747947735985782e-5_f64 * t9017 - 0.2698425785107458272e-5_f64 * t9021 - 0.15176747947735985782e-6_f64 * t9024 + 0.2698425785107458272e-6_f64 * t9027 + 0.14648281543675415196e-4_f64 * t9032 - 0.4637672555408563478e-4_f64 * t9034 + 0.11272120794395814009e-6_f64 * t9036 - 0.20041830772435757309e-6_f64 * t9038 + 0.11255061864162936194e-7_f64 * t9042 + 0.11255061864162936194e-6_f64 * t9044;
    t10679
}
