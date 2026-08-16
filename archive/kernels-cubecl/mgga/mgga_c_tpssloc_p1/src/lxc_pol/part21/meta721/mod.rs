//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta721 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2565;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2566;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2567;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2568;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta721<F: Float>(t14536: F, t225: F, t10164: F, t1634: F, t14532: F, t14562: F, t14527: F, t14534: F, t11190: F, t1670: F, t3242: F, t457: F, t2394: F, t4734: F, t14707: F, t690: F, t1654: F, t9698: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t50625, t50628, t50632, t50653, t50690, t50703, t50819, t50822) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2565::<F>(t14536, t225, t10164, t1634, t14532, t14562, t14527, t14534, t11190, t1670, t3242, t457);
        let t50826 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2566::<F>(t2394, t4734);
        let t50828 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2567::<F>(t14707, t690);
        let t50834 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2568::<F>(t1654, t9698);
    (t50625, t50628, t50632, t50653, t50690, t50703, t50819, t50822, t50826, t50828, t50834)
}
