//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1806;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1807;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta471<F: Float>(t2379: F, t28: F, t2752: F, t13487: F, t1081: F, t776: F, t2553: F, t2749: F, t868: F, t2745: F, t1877: F, t1915: F, t22959: F, t23286: F, t23290: F, t23295: F, t2522: F, t3231: F, t4314: F, t6666: F, t6670: F, t6841: F, t6848: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t23781, t23788) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1806::<F>(t2379, t28, t2752);
        let (t23789, t23792, t23796, t23807, t23810, t23813, t23820) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1807::<F>(t13487, t23788, t1081, t776, t2553, t28, t2749, t868, t2745, t1877, t1915, t22959, t23286, t23290, t23295, t23781, t2522, t3231, t4314, t6666, t6670, t6841, t6848);
    (t23781, t23788, t23789, t23792, t23796, t23807, t23810, t23813, t23820)
}
