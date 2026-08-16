//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta212 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk855;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta212<F: Float>(t13004: F, t205: F, t1489: F, t9541: F, t4126: F, t782: F, t4134: F, t9546: F, t1496: F, t2528: F, t4199: F, t2663: F, t4211: F, t2535: F, t1471: F, t32: F, t118: F, t1474: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13005, t13010, t13012, t13022, t13087, t13107, t13109) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk855::<F>(t13004, t205, t1489, t9541, t4126, t782, t4134, t9546, t1496, t2528, t4199, t2663, t4211);
        let (t13113, t13115, t13123) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk856::<F>(t2535, t4199, t1471, t32, t118, t1474);
    (t13005, t13010, t13012, t13022, t13087, t13107, t13109, t13113, t13115, t13123)
}
