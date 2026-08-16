//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta282 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1431;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta282<F: Float>(t12997: F, t792: F, t12984: F, t686: F, t776: F, t131: F, t9558: F, t205: F, t1489: F, t9541: F, t4126: F, t782: F) -> (F, F, F, F, F, F, F) {
        let (t12998, t13000, t13002, t13004, t13005, t13010, t13012) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1431::<F>(t12997, t792, t12984, t686, t776, t131, t9558, t205, t1489, t9541, t4126, t782);
    (t12998, t13000, t13002, t13004, t13005, t13010, t13012)
}
