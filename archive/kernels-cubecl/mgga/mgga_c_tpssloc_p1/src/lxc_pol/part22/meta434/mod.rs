//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta434 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1770;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1771;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta434<F: Float>(t5456: F, t649: F, t5465: F, t626: F, t5464: F, t9365: F, t666: F, t4043: F, t4067: F, t5489: F, t2331: F, t5488: F, t5468: F, t9384: F, t659: F, t1444: F, t2: F, t584: F, t2341: F, t5396: F, t9212: F, t95: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19461, t19471, t19473, t19474, t19477, t19480, t19482) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1770::<F>(t5456, t649, t5465, t626, t5464, t9365, t666, t4043, t4067, t5489, t2331, t5488);
        let (t19483, t19489, t19493, t19499, t19503, t19504) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1771::<F>(t19482, t666, t5468, t9384, t659, t1444, t2, t584, t2341, t5396, t9212, t95);
    (t19461, t19471, t19473, t19474, t19477, t19480, t19483, t19489, t19493, t19499, t19503, t19504)
}
