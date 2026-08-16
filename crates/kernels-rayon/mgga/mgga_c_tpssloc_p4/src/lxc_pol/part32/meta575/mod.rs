//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1950;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta575(t5371: f64, t7467: f64, t5456: f64, t576: f64, t1873: f64, t1458: f64, t3941: f64, t5493: f64, t1401: f64, t28017: f64, t1409: f64, t22510: f64, t24498: f64, t27356: f64, t5392: f64, t5398: f64, t5415: f64, t56: f64, t7251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28892, t28893, t28895, t28896, t28898, t28899, t28901, t28903, t29473) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1950(t5371, t7467, t5456, t576, t1873, t1458, t3941, t5493, t1401, t28017, t1409, t22510, t24498, t27356, t5392, t5398, t5415, t56, t7251);
    (t28892, t28893, t28895, t28896, t28898, t28899, t28901, t28903, t29473)
}
