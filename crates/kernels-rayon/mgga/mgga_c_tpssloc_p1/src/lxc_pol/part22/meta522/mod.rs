//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta522(t11915: f64, t22348: f64, t1734: f64, t1932: f64, t475: f64, t6260: f64, t11883: f64, t11889: f64, t1751: f64, t6224: f64, t3612: f64, t6218: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22349, t22354, t22355, t22358, t22361, t22364, t22365, t22368) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1988(t11915, t22348, t1734, t1932, t475, t6260, t11883, t11889, t1751, t6224, t3612, t6218);
    (t22349, t22354, t22355, t22358, t22361, t22364, t22365, t22368)
}
