//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1950;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta625(t1985: f64, t6907: f64, t90739: f64, t22685: f64, t22686: f64, t26193: f64, t16018: f64, t6888: f64, t6889: f64, t6890: f64, t22674: f64, t22892: f64, t26189: f64, t1324: f64, t254: f64, t22724: f64, t26344: f64, t22643: f64, t7691: f64, t81195: f64, t1388: f64, t25988: f64, t1845: f64, t3719: f64, t22573: f64, t7684: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91469, t91478, t91482, t91486) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1950(t1985, t6907, t90739, t22685, t22686, t26193, t16018, t6888, t6889, t6890, t22674, t22892, t26189);
        let (t91505, t91531, t91548, t91565, t91603, t91655) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1951(t1324, t254, t22724, t26344, t22643, t7691, t81195, t1388, t25988, t1845, t3719, t22573, t7684);
    (t91469, t91478, t91482, t91486, t91505, t91531, t91548, t91565, t91603, t91655)
}
