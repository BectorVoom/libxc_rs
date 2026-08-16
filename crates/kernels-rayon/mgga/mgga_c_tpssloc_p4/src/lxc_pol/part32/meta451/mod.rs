//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1716;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1717;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta451(t1878: f64, t557: f64, t556: f64, t598: f64, t213: f64, t281: f64, t6931: f64, t1351: f64, t22705: f64, t236: f64, t550: f64, t2003: f64, t3862: f64, t1358: f64, t6940: f64, t1887: f64, t22715: f64, t534: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22839, t22842, t22843, t22844, t22845, t22852) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1716(t1878, t557, t556, t598, t213, t281, t6931);
        let (t22855, t22856, t22859, t22860, t22863) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1717(t1351, t22705, t236, t550, t22852, t2003, t3862, t1358, t6940, t1887, t22715, t534);
    (t22839, t22842, t22843, t22844, t22845, t22852, t22855, t22856, t22859, t22860, t22863)
}
