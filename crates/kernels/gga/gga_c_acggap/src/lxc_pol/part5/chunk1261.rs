//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1261/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1261<F: Float>(t14047: F, t6090: F, t1137: F, t5594: F, t1809: F, t3573: F, t3409: F, t5792: F, t1165: F, t17857: F, t17859: F, t17861: F, t17868: F, t17870: F, t17876: F, t17886: F, t17891: F, t3396: F, t4417: F, t4752: F) -> F {
    let t23255 = t14047 * t6090;
    let t23263 = t1137 * t5594;
    let t23265 = t3573 * t1809;
    let t23269 = t3409 * t5792;
    let t23271 = -F::new(0.20579528696673473748e-1) * t3396 * t1165 * t4417 * t4752 - F::new(0.68598428988911579156e-2) * t23255 + F::new(0.64025200389650807212e-1) * t17857 - F::new(0.32012600194825403606e-1) * t17859 + F::new(0.48018900292238105408e-1) * t17861 + F::new(0.85748036236139473944e-3) * t17868 + F::new(0.16006300097412701803e0) * t17870 - F::new(0.64025200389650807212e-1) * t17876 + F::new(7.0) / F::new(36.0) * t23263 - F::new(35.0) / F::new(216.0) * t23265 - F::new(0.16006300097412701803e-1) * t17886 + F::new(0.51448821741683684367e-1) * t17891 + F::new(0.80031500487063509014e-2) * t23269;
    t23271
}
