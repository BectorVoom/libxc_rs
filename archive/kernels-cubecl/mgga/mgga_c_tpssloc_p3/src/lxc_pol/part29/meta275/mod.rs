//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1279;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta275<F: Float>(t533: F, t7752: F, t1390: F, t1983: F, t2019: F, t5161: F, t1873: F, t5371: F, t1458: F) -> (F, F, F, F, F, F, F) {
        let (t7753, t7754, t7755, t7756, t7757, t7768, t7769) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1279::<F>(t533, t7752, t1390, t1983, t2019, t5161, t1873, t5371, t1458);
    (t7753, t7754, t7755, t7756, t7757, t7768, t7769)
}
