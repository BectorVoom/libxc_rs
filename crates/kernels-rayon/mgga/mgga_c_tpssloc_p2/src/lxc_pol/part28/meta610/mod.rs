//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1921;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta610(t22724: f64, t26436: f64, t1307: f64, t1352: f64, t1834: f64, t22633: f64, t6976: f64, t16037: f64, t1992: f64, t22897: f64, t26423: f64, t81159: f64, t215: f64, t22839: f64, t562: f64, t80854: f64, t16226: f64, t22685: f64, t26395: f64, t3734: f64, t6637: f64, t16125: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90900, t90907, t90910, t90912) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1921(t22724, t26436, t1307, t1352, t1834, t22633, t6976, t16037, t1992, t22897, t26423, t81159);
        let (t90915, t90917, t90921, t90929) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1922(t215, t22839, t562, t80854, t16226, t22685, t26395, t3734, t6637, t16125, t1992, t6976);
    (t90900, t90907, t90910, t90912, t90915, t90917, t90921, t90929)
}
