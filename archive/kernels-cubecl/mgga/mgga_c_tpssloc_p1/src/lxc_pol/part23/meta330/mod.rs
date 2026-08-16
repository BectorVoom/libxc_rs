//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta330<F: Float>(t1441: F, t1458: F, t1799: F, t1824: F, t1484: F, t1530: F, t1409: F, t1615: F, t1845: F, t5456: F, t576: F, t460: F, t6144: F) -> (F, F, F, F, F, F, F) {
        let (t28002, t28099, t28248, t28651, t28830, t28893, t29614) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1098::<F>(t1441, t1458, t1799, t1824, t1484, t1530, t1409, t1615, t1845, t5456, t576, t460, t6144);
    (t28002, t28099, t28248, t28651, t28830, t28893, t29614)
}
