//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1564;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1565;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1566;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta404<F: Float>(t1864: F, t645: F, t192: F, t532: F, t1982: F, t3701: F, t3914: F, t1390: F, t3719: F, t3734: F, t191: F, t3660: F, t1887: F, t6916: F, t213: F, t225: F, t562: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t22550, t22573, t22574) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1564::<F>(t1864, t645, t192, t532, t1982);
        let (t22578, t22584, t22596, t22607, t22633) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1565::<F>(t3701, t3914, t1390, t3719, t3734, t191, t192, t3660, t1887, t6916);
        let t22635 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1566::<F>(t213, t225, t562);
    (t22550, t22573, t22574, t22578, t22584, t22596, t22607, t22633, t22635)
}
