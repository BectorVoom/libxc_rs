//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1261/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1261(t14047: f64, t6090: f64, t1137: f64, t5594: f64, t1809: f64, t3573: f64, t3409: f64, t5792: f64, t1165: f64, t17857: f64, t17859: f64, t17861: f64, t17868: f64, t17870: f64, t17876: f64, t17886: f64, t17891: f64, t3396: f64, t4417: f64, t4752: f64) -> f64 {
    let t23255 = t14047 * t6090;
    let t23263 = t1137 * t5594;
    let t23265 = t3573 * t1809;
    let t23269 = t3409 * t5792;
    let t23271 = -0.20579528696673473748e-1_f64 * t3396 * t1165 * t4417 * t4752 - 0.68598428988911579156e-2_f64 * t23255 + 0.64025200389650807212e-1_f64 * t17857 - 0.32012600194825403606e-1_f64 * t17859 + 0.48018900292238105408e-1_f64 * t17861 + 0.85748036236139473944e-3_f64 * t17868 + 0.16006300097412701803e0_f64 * t17870 - 0.64025200389650807212e-1_f64 * t17876 + 7.0_f64 / 36.0_f64 * t23263 - 35.0_f64 / 216.0_f64 * t23265 - 0.16006300097412701803e-1_f64 * t17886 + 0.51448821741683684367e-1_f64 * t17891 + 0.80031500487063509014e-2_f64 * t23269;
    t23271
}
