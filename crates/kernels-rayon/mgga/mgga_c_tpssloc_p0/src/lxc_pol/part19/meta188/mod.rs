//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta188 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk844;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk845;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta188(t10021: f64, t241: f64, t244: f64, t248: f64, t238: f64, t154: f64, t9569: f64, t222: f64, t2606: f64, t9573: f64, t119: f64, t210: f64, t9458: f64, t805: f64, t9541: f64, t2563: f64, t2610: f64, t9516: f64, t10009: f64, t10012: f64, t10014: f64, t10017: f64, t249: f64, t2643: f64, t787: f64, t9559: f64, t10006: f64, t2623: f64, t2707: f64, t4178: f64, t831: f64, t843: f64, t9602: f64, t9604: f64, t9609: f64, t9613: f64, t9618: f64, t9623: f64, t9629: f64, t9634: f64, t9639: f64, t9963: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t10022, t10024, t10026, t10027, t10029, t10030, t10033) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk844(t10021, t241, t244, t248, t238, t154, t9569, t222, t2606, t9573, t119, t210, t9458);
        let (t10041, t10044) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk845(t805, t9541, t2563, t2610, t119, t210, t9516, t10009, t10012, t10014, t10017, t10026, t10029, t10030, t10033, t249, t2643, t787, t9559);
        let t10046 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk846(t10006, t10044, t2623, t2643, t2707, t4178, t831, t843, t9602, t9604, t9609, t9613, t9618, t9623, t9629, t9634, t9639, t9963);
    (t10022, t10024, t10027, t10033, t10041, t10046)
}
