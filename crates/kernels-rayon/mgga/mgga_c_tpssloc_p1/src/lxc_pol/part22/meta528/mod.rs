//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1998;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1999;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta528(t1215: f64, t1409: f64, t254: f64, t492: f64, t1254: f64, t1763: f64, t1441: f64, t1458: f64, t343: f64, t5842: f64, t5456: f64, t576: f64, t460: f64, t6144: f64, t64: f64, t9365: f64, t20: f64, t60: f64, t9108: f64, t94: f64, t102: f64, t9174: f64, t2: f64, t591: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27524, t27784, t27843, t28002, t28565, t28893) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1998(t1215, t1409, t254, t492, t1254, t1763, t1441, t1458, t343, t5842, t5456, t576);
        let (t29614, t29903, t32253, t35577, t35761, t39031) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1999(t460, t6144, t64, t9365, t20, t60, t9108, t94, t102, t9174, t2, t591);
    (t27524, t27784, t27843, t28002, t28565, t28893, t29614, t29903, t32253, t35577, t35761, t39031)
}
