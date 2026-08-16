//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1909;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1910;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta604(t22716: f64, t7701: f64, t1834: f64, t212: f64, t22642: f64, t6890: f64, t1373: f64, t254: f64, t26215: f64, t81228: f64, t81326: f64, t16436: f64, t1985: f64, t6889: f64, t6906: f64, t6897: f64, t6907: f64, t90544: f64, t22662: f64, t26193: f64, t26203: f64, t6883: f64, t7700: f64, t80645: f64, t22633: f64, t22635: f64, t26214: f64, t3719: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90659, t90663, t90665, t90686, t90690) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1909(t22716, t7701, t1834, t212, t22642, t6890, t1373, t254, t26215, t81228, t81326, t16436, t1985, t6889, t6906);
        let (t90701, t90704, t90707, t90723, t90728) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1910(t6897, t6907, t90544, t1985, t22662, t26193, t26203, t6883, t7700, t80645, t22633, t22635, t26214, t3719);
    (t90659, t90663, t90665, t90686, t90690, t90701, t90704, t90707, t90723, t90728)
}
