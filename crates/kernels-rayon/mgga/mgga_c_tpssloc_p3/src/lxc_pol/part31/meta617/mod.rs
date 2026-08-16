//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1865;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta617(t1985: f64, t22666: f64, t28205: f64, t7700: f64, t90739: f64, t28206: f64, t6883: f64, t1385: f64, t1992: f64, t22635: f64, t3886: f64, t6460: f64, t22674: f64, t6897: f64, t22892: f64, t28209: f64, t22685: f64, t28191: f64, t6888: f64, t19631: f64, t6889: f64, t6890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96857, t96866, t96868, t96873) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1865(t1985, t22666, t28205, t7700, t90739, t28206, t6883, t1385, t1992, t22635, t3886, t6460);
        let (t96878, t96893, t96896, t96900, t96905) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1866(t22674, t28205, t6897, t22892, t28209, t22666, t22685, t28191, t6888, t19631, t6889, t6890);
    (t96857, t96866, t96868, t96873, t96878, t96893, t96896, t96900, t96905)
}
