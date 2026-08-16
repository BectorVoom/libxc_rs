//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1408;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1409;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta447(t13665: f64, t9863: f64, t9866: f64, t9575: f64, t9572: f64, t3863: f64, t5569: f64, t3860: f64, t5571: f64, t9419: f64, t1882: f64, t4010: f64, t1885: f64, t46722: f64, t1389: f64, t46856: f64, t543: f64, t685: f64, t72: f64, t13955: f64, t46946: f64, t47198: f64, t5665: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48304, t48306, t48313, t48324, t48331, t48333, t48335, t48455) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1408(t13665, t9863, t9866, t9575, t9572, t3863, t5569, t3860, t5571, t9419, t1882, t4010);
        let (t48518, t48563, t48600, t48792) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1409(t1885, t46722, t1389, t1882, t46856, t543, t685, t72, t13955, t46946, t47198, t5665);
    (t48304, t48306, t48313, t48324, t48331, t48333, t48335, t48455, t48518, t48563, t48600, t48792)
}
