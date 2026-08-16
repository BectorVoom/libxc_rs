//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1832;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta421(t14255: f64, t291: f64, t10629: f64, t1580: f64, t10632: f64, t2906: f64, t959: f64, t1573: f64, t2904: f64, t4408: f64, t923: f64, t1561: f64, t2885: f64, t2860: f64, t10760: f64, t13517: f64, t13519: f64, t13522: f64, t13524: f64, t13526: f64, t13657: f64, t1569: f64, t2863: f64, t2881: f64, t2889: f64, t2907: f64, t4411: f64, t933: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14257, t14258, t14259, t14260, t14262, t14263, t14266, t14271) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1832(t14255, t291, t10629, t1580, t10632, t2906, t959, t1573, t2904, t4408, t923, t1561, t2885);
        let (t14276, t14279) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1833(t1561, t2860, t10760, t13517, t13519, t13522, t13524, t13526, t13657, t14263, t14266, t14271, t1569, t2863, t2881, t2889, t2907, t4411, t933);
    (t14257, t14258, t14259, t14260, t14262, t14263, t14266, t14271, t14276, t14279)
}
