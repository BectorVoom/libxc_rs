//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1914;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta555(t6330: f64, t6890: f64, t6889: f64, t22685: f64, t26193: f64, t7700: f64, t1985: f64, t225: f64, t567: f64, t6434: f64, t214: f64, t6460: f64, t6906: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28191, t28192, t28193, t28195, t28196, t28199, t28200, t28201, t28205) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1914(t6330, t6890, t6889, t22685, t26193, t7700, t1985, t225, t567, t6434, t214, t6460, t6906);
    (t28191, t28192, t28193, t28195, t28196, t28199, t28200, t28201, t28205)
}
