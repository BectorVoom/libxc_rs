//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta232 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1009;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta232<F: Float>(t1323: F, t1834: F, t1811: F, t3726: F, t1307: F, t1810: F, t210: F, t119: F, t5187: F, t225: F, t5210: F, t554: F, t1814: F, t68: F) -> (F, F, F, F, F, F, F, F) {
        let (t5217, t5220, t5223, t5226, t5227, t5230) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1009::<F>(t1323, t1834, t1811, t3726, t1307, t1810, t210, t119, t5187, t225, t5210);
        let (t5231, t5234) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1010::<F>(t5230, t554, t1814, t68);
    (t5217, t5220, t5223, t5226, t5227, t5230, t5231, t5234)
}
