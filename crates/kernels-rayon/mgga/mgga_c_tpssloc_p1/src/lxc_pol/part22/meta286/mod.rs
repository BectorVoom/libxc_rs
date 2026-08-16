//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1437;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1438;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1439;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta286(t13176: f64, t816: f64, t1512: f64, t9671: f64, t2697: f64, t4257: f64, t2563: f64, t4159: f64, t4155: f64, t9573: f64, t2644: f64, t820: f64, t1509: f64, t828: f64, t2632: f64, t1500: f64, t2693: f64, t4163: f64, t838: f64, t120: f64, t4233: f64, t2642: f64, t4166: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13177, t13182, t13190, t13202, t13208, t13222) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1437(t13176, t816, t1512, t9671, t2697, t4257, t2563, t4159, t4155, t9573, t2644, t820);
        let (t13223, t13228) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1438(t1509, t828, t2632);
        let (t13234, t13237, t13242, t13251) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1439(t1500, t2693, t4163, t838, t120, t4233, t2642, t4166);
    (t13177, t13182, t13190, t13202, t13208, t13222, t13223, t13228, t13234, t13237, t13242, t13251)
}
