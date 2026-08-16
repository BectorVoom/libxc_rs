//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta429 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1719;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1720;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1721;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta429<F: Float>(t2020: F, t22607: F, t2314: F, t6535: F, t12823: F, t1874: F, t4034: F, t6525: F, t12734: F, t2006: F, t3752: F, t1323: F, t6955: F, t2015: F, t3888: F, t12021: F, t1887: F, t6916: F, t213: F, t225: F, t562: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22608, t22610, t22612, t22614, t22616, t22618, t22622, t22624) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1719::<F>(t2020, t22607, t2314, t6535, t12823, t1874, t4034, t6525, t12734, t2006, t3752, t1323, t6955);
        let (t22630, t22633) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1720::<F>(t2015, t3888, t12021, t1887, t6916);
        let t22635 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1721::<F>(t213, t225, t562);
    (t22608, t22610, t22612, t22614, t22616, t22618, t22622, t22624, t22630, t22633, t22635)
}
