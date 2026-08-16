//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1903;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta601(t22633: f64, t22635: f64, t26337: f64, t3911: f64, t26206: f64, t6883: f64, t1834: f64, t794: f64, t22892: f64, t6891: f64, t22704: f64, t26355: f64, t81326: f64, t26197: f64, t80670: f64, t1307: f64, t26331: f64, t5187: f64, t567: f64, t26332: f64, t3719: f64, t213: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90539, t90541, t90544, t90546, t90549) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1903(t22633, t22635, t26337, t3911, t26206, t6883, t1834, t794, t22892, t6891, t22704, t26355, t81326);
        let (t90551, t90556, t90560, t90566) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1904(t26197, t80670, t1307, t22635, t26331, t5187, t567, t26332, t3719, t1834, t213, t225);
    (t90539, t90541, t90544, t90546, t90549, t90551, t90556, t90560, t90566)
}
