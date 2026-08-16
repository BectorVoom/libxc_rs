//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta492 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1918;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta492<F: Float>(t10482: F, t21390: F, t1021: F, t248: F, t3131: F, t360: F, t10278: F, t20234: F, t2979: F, t21122: F, t4510: F, t13769: F, t17863: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21391, t21393, t21396, t21398, t21403, t21405, t21409, t21410, t21413, t21416) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1918::<F>(t10482, t21390, t1021, t248, t3131, t360, t10278, t20234, t2979, t21122, t4510, t13769, t17863);
    (t21391, t21393, t21396, t21398, t21403, t21405, t21409, t21410, t21413, t21416)
}
