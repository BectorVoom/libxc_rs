//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1135;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta265(t6889: f64, t7700: f64, t1985: f64, t1811: f64, t6916: f64, t1799: f64, t236: f64, t1998: f64, t6926: f64, t1339: f64, t1825: f64, t6936: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7701, t7702, t7706, t7708, t7709, t7710, t7712, t7713) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1135(t6889, t7700, t1985, t1811, t6916, t1799, t236, t1998, t6926, t1339, t1825, t6936);
    (t7701, t7702, t7706, t7708, t7709, t7710, t7712, t7713)
}
