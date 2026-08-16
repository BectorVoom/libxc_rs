//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta593 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2109;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta593<F: Float>(t47093: F, t4159: F, t9541: F, t1516: F, t41052: F, t4166: F, t9600: F, t849: F, t13176: F, t2696: F, t1509: F, t9975: F, t242: F, t41347: F, t812: F, t2627: F, t4265: F, t226: F, t40931: F, t68: F, t2394: F, t4344: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t47094, t47231, t47270, t47275, t47277, t47278, t47285) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2109::<F>(t47093, t4159, t9541, t1516, t41052, t4166, t9600, t849, t13176, t2696, t1509, t9975);
        let (t47307, t47374, t47386, t47705) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2110::<F>(t242, t41347, t812, t2627, t4265, t226, t40931, t68, t2394, t4344);
    (t47094, t47231, t47270, t47275, t47277, t47278, t47285, t47307, t47374, t47386, t47705)
}
