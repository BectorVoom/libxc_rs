//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta188 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk844;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk845;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta188<F: Float>(t10021: F, t241: F, t244: F, t248: F, t238: F, t154: F, t9569: F, t222: F, t2606: F, t9573: F, t119: F, t210: F, t9458: F, t805: F, t9541: F, t2563: F, t2610: F, t9516: F, t10009: F, t10012: F, t10014: F, t10017: F, t249: F, t2643: F, t787: F, t9559: F, t10006: F, t2623: F, t2707: F, t4178: F, t831: F, t843: F, t9602: F, t9604: F, t9609: F, t9613: F, t9618: F, t9623: F, t9629: F, t9634: F, t9639: F, t9963: F) -> (F, F, F, F, F, F) {
        let (t10022, t10024, t10026, t10027, t10029, t10030, t10033) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk844::<F>(t10021, t241, t244, t248, t238, t154, t9569, t222, t2606, t9573, t119, t210, t9458);
        let (t10041, t10044) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk845::<F>(t805, t9541, t2563, t2610, t119, t210, t9516, t10009, t10012, t10014, t10017, t10026, t10029, t10030, t10033, t249, t2643, t787, t9559);
        let t10046 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk846::<F>(t10006, t10044, t2623, t2643, t2707, t4178, t831, t843, t9602, t9604, t9609, t9613, t9618, t9623, t9629, t9634, t9639, t9963);
    (t10022, t10024, t10027, t10033, t10041, t10046)
}
