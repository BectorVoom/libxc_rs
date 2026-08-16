//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1253;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1254;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1255;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta277(t1998: f64, t7708: f64, t6926: f64, t1339: f64, t1825: f64, t6936: f64, t1814: f64, t2002: f64, t559: f64, t1827: f64, t6945: f64, t1831: f64, t6952: f64, t6915: f64, t6922: f64, t6935: f64, t6949: f64, t7706: f64, t539: f64, t1842: f64, t2015: f64, t3887: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t7709, t7710, t7712, t7713, t7715, t7716, t7718, t7720) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1253(t1998, t7708, t6926, t1339, t1825, t6936, t1814, t2002, t559, t1827, t6945, t1831, t6952);
        let t7722 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1254(t6915, t6922, t6935, t6949, t7706, t7710, t7713, t7716, t7718, t7720);
        let (t7723, t7729) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1255(t539, t7722, t1842, t2015, t3887);
    (t7709, t7712, t7715, t7722, t7723, t7729)
}
