//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1271;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta280(t6889: f64, t7700: f64, t1985: f64, t1807: f64, t2006: f64, t1811: f64, t6916: f64, t1799: f64, t236: f64) -> (f64, f64, f64, f64, f64) {
        let (t7701, t7702, t7704, t7706, t7708) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1271(t6889, t7700, t1985, t1807, t2006, t1811, t6916, t1799, t236);
    (t7701, t7702, t7704, t7706, t7708)
}
