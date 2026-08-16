//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1915;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta556(t28205: f64, t6889: f64, t1985: f64, t6347: f64, t6890: f64, t6888: f64, t26193: f64, t7691: f64, t1842: f64, t7749: f64, t3887: f64, t2015: f64, t6439: f64, t12021: f64, t22933: f64, t1375: f64, t1843: f64, t20060: f64, t2016: f64, t22924: f64, t22926: f64, t26366: f64, t26475: f64, t27067: f64, t28193: f64, t28196: f64, t28201: f64, t5321: f64, t6440: f64, t6958: f64, t7729: f64, t7750: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28206, t28207, t28209, t28210, t28211, t28213, t28214, t28220, t28223) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1915(t28205, t6889, t1985, t6347, t6890, t6888, t26193, t7691, t1842, t7749, t3887, t2015, t6439);
        let (t28224, t28232, t28233, t28236) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1916(t12021, t28223, t22933, t6439, t6889, t1985, t1375, t1843, t20060, t2016, t22924, t22926, t26366, t26475, t27067, t28193, t28196, t28201, t28207, t28211, t28214, t28220, t5321, t6440, t6958, t7729, t7750);
    (t28206, t28209, t28210, t28213, t28220, t28224, t28232, t28233, t28236)
}
