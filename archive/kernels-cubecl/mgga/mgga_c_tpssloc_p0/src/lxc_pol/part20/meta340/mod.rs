//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta340<F: Float>(t1372: F, t3752: F, t1376: F, t68: F, t1385: F, t3888: F, t3911: F, t3887: F, t225: F, t3753: F, t3880: F, t1323: F, t3879: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12016, t12019, t12020, t12021, t12022, t12023, t12026, t12027, t12030, t12033, t12036) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1639::<F>(t1372, t3752, t1376, t68, t1385, t3888, t3911, t3887, t225, t3753, t3880, t1323, t3879);
    (t12016, t12019, t12020, t12021, t12022, t12023, t12026, t12027, t12030, t12033, t12036)
}
