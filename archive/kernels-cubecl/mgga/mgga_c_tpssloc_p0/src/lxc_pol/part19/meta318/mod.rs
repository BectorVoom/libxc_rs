//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1130;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1131;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta318<F: Float>(t39514: F, t677: F, t9919: F, t3684: F, t2393: F, t2535: F, t12110: F, t9882: F, t12466: F, t3719: F, t3918: F, t39483: F, t39490: F, t39492: F, t39496: F, t39499: F, t39502: F, t39505: F, t39508: F, t39511: F, t39513: F, t2420: F, t701: F, t9778: F) -> (F, F, F, F, F, F, F, F) {
        let (t39515, t39516, t39518, t39519, t39521, t39523, t39524) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1130::<F>(t39514, t677, t9919, t3684, t2393, t2535, t12110, t9882, t12466, t3719, t3918, t39483, t39490, t39492, t39496, t39499, t39502, t39505, t39508, t39511, t39513);
        let t39529 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1131::<F>(t2420, t701, t9778);
    (t39515, t39516, t39518, t39519, t39521, t39523, t39524, t39529)
}
