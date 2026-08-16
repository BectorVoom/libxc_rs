//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1269;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1270;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta279(t2020: f64, t7685: f64, t1390: f64, t1799: f64, t6878: f64, t1983: f64, t6890: f64, t6889: f64, t6888: f64, t1834: f64, t225: f64, t567: f64, t214: f64, t1985: f64, t1842: f64, t6906: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7686, t7687, t7688, t7690, t7691) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1269(t2020, t7685, t1390, t1799, t6878, t1983, t6890);
        let (t7692, t7693, t7696, t7697, t7698, t7700) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1270(t6889, t7691, t6888, t1834, t225, t567, t214, t1985, t1842, t6906);
    (t7686, t7687, t7688, t7690, t7691, t7692, t7693, t7696, t7697, t7698, t7700)
}
