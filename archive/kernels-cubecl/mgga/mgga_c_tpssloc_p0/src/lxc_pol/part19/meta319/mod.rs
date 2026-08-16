//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1132;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta319<F: Float>(t12088: F, t2528: F, t3691: F, t9919: F, t2367: F, t2508: F, t39378: F, t9493: F, t1294: F, t9713: F, t2405: F, t2412: F, t9479: F, t9481: F, t39273: F, t39275: F, t39278: F, t39281: F, t39284: F, t39289: F, t39291: F, t39293: F, t39295: F, t39298: F, t683: F, t702: F) -> (F, F, F, F, F, F, F, F) {
        let (t39531, t39533, t39535, t39537, t39539, t39541, t39549) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1132::<F>(t12088, t2528, t3691, t9919, t2367, t2508, t39378, t9493, t1294, t9713, t2405, t2412, t9479, t9481);
        let t39563 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1133::<F>(t39273, t39275, t39278, t39281, t39284, t39289, t39291, t39293, t39295, t39298, t683, t702);
    (t39531, t39533, t39535, t39537, t39539, t39541, t39549, t39563)
}
