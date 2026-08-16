//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2345;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2346;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta595<F: Float>(t1799: F, t3792: F, t6414: F, t1484: F, t2632: F, t5611: F, t154: F, t2558: F, t10: F, t2229: F, t116: F, t117: F, t556: F, t243: F, t3008: F, t343: F, t3034: F, t371: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20468, t20473, t20981, t20986, t22715, t22811, t22815) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2345::<F>(t1799, t3792, t6414, t1484, t2632, t5611, t154, t2558, t10, t2229, t116, t117);
        let (t22843, t23076, t23494, t23508) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2346::<F>(t556, t243, t3008, t343, t3034, t371);
    (t20468, t20473, t20981, t20986, t22715, t22811, t22815, t22843, t23076, t23494, t23508)
}
