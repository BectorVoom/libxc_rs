//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta342 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1124;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1125;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta342<F: Float>(t3684: F, t39503: F, t677: F, t9722: F, t9919: F, t2393: F, t2535: F, t2420: F, t701: F, t9778: F, t2367: F, t2508: F, t39378: F, t9493: F, t1294: F, t2405: F, t2412: F, t9479: F, t9481: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39505, t39506, t39508, t39516, t39518, t39519, t39521, t39529) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1124::<F>(t3684, t39503, t677, t9722, t9919, t2393, t2535, t2420, t701, t9778);
        let (t39535, t39537, t39539, t39549) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1125::<F>(t2367, t2508, t39378, t9493, t1294, t2405, t2412, t9479, t9481);
    (t39505, t39506, t39508, t39516, t39518, t39519, t39521, t39529, t39535, t39537, t39539, t39549)
}
