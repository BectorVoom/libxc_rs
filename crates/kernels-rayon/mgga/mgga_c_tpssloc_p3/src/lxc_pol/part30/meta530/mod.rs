//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1872;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta530(t1878: f64, t22683: f64, t221: f64, t5308: f64, t22844: f64, t6604: f64, t1361: f64, t1339: f64, t5287: f64, t6936: f64, t22779: f64, t7712: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26284, t26285, t26286, t26288, t26289, t26290, t26292, t26293, t26295) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1872(t1878, t22683, t221, t5308, t22844, t6604, t1361, t1339, t5287, t6936, t22779, t7712);
    (t26284, t26285, t26286, t26288, t26289, t26290, t26292, t26293, t26295)
}
