//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1998;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1999;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta528<F: Float>(t1215: F, t1409: F, t254: F, t492: F, t1254: F, t1763: F, t1441: F, t1458: F, t343: F, t5842: F, t5456: F, t576: F, t460: F, t6144: F, t64: F, t9365: F, t20: F, t60: F, t9108: F, t94: F, t102: F, t9174: F, t2: F, t591: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27524, t27784, t27843, t28002, t28565, t28893) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1998::<F>(t1215, t1409, t254, t492, t1254, t1763, t1441, t1458, t343, t5842, t5456, t576);
        let (t29614, t29903, t32253, t35577, t35761, t39031) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1999::<F>(t460, t6144, t64, t9365, t20, t60, t9108, t94, t102, t9174, t2, t591);
    (t27524, t27784, t27843, t28002, t28565, t28893, t29614, t29903, t32253, t35577, t35761, t39031)
}
