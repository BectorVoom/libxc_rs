//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1076;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta295<F: Float>(t649: F, t671: F, t157: F, t9929: F, t2379: F, t262: F, t9897: F, t2570: F, t67: F, t792: F, t131: F, t9558: F, t205: F, t4126: F, t782: F, t68: F, t822: F, t2644: F, t820: F, t2617: F, t4177: F, t2628: F, t836: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12734, t12908, t12935, t12939, t12998, t13004) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1076::<F>(t649, t671, t157, t9929, t2379, t262, t9897, t2570, t67, t792, t131, t9558);
        let (t13005, t13012, t13151, t13222, t13254, t13257) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1077::<F>(t13004, t205, t4126, t782, t68, t822, t2644, t820, t2617, t4177, t2628, t836);
    (t12734, t12908, t12935, t12939, t12998, t13005, t13012, t13151, t13222, t13254, t13257)
}
