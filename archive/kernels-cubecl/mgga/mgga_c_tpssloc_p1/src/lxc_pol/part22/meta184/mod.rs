//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta184 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1099;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1100;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1101;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta184<F: Float>(t25: F, t28: F, t5397: F, zeta_threshold: F, t31: F, t65: F, t1410: F, t1426: F, t2267: F, t5392: F, t43: F, t48: F, t480: F, sigma2: F, t2274: F, t55: F, t1420: F, t1423: F, t2282: F, t39: F, t51: F, t56: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t5398 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1099::<F>(t25, t28, t5397, zeta_threshold);
        let (t5399, t5400, t5403, t5408, t5411, t5415, t5416) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1100::<F>(t31, t5398, t65, t1410, t1426, t2267, t5392, t43, t48, t480, sigma2);
        let (t5421, t5424, t5427) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1101::<F>(t2274, t5392, t5398, t55, t1420, t1423, t2282, t39, t51, t5408, t5411, t5416, t56);
    (t5398, t5399, t5400, t5403, t5408, t5411, t5415, t5416, t5421, t5424, t5427)
}
