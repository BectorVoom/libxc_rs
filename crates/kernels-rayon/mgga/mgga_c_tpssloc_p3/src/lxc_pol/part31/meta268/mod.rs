//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta268 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1114;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1115;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta268(t6889: f64, t7691: f64, t6888: f64, t1834: f64, t225: f64, t567: f64, t214: f64, t1985: f64, t1842: f64, t6906: f64, t1811: f64, t6916: f64, t1799: f64, t236: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7692, t7693, t7696, t7697, t7698, t7700) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1114(t6889, t7691, t6888, t1834, t225, t567, t214, t1985, t1842, t6906);
        let (t7701, t7702, t7706, t7708) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1115(t6889, t7700, t1985, t1811, t6916, t1799, t236);
    (t7692, t7693, t7696, t7697, t7698, t7700, t7701, t7702, t7706, t7708)
}
