//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1866;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1867;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta394(t14255: f64, t291: f64, t10629: f64, t1580: f64, t10632: f64, t2906: f64, t959: f64, t1573: f64, t2904: f64, t4408: f64, t923: f64, t1561: f64, t2885: f64, t2860: f64, t10760: f64, t13517: f64, t13519: f64, t13522: f64, t13524: f64, t13526: f64, t13657: f64, t1569: f64, t2863: f64, t2881: f64, t2889: f64, t2907: f64, t4411: f64, t933: f64, t13550: f64, t13563: f64, t10296: f64, t10298: f64, t10302: f64, t13566: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14257, t14259, t14260, t14262, t14263, t14266, t14271) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1866(t14255, t291, t10629, t1580, t10632, t2906, t959, t1573, t2904, t4408, t923, t1561, t2885);
        let (t14276, t14279) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1867(t1561, t2860, t10760, t13517, t13519, t13522, t13524, t13526, t13657, t14263, t14266, t14271, t1569, t2863, t2881, t2889, t2907, t4411, t933);
        let (t14287, t14291, t14304) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1868(t13550, t13563, t10296, t10298, t10302, t13566, t13569, t13572, t13575, t13578, t13581, t13584, t13587);
    (t14257, t14259, t14260, t14262, t14263, t14266, t14271, t14276, t14279, t14287, t14291, t14304)
}
