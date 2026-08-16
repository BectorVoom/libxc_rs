//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2187;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta605<F: Float>(t1174: F, t11760: F, t135: F, t11147: F, t3439: F, t11789: F, t820: F, t3577: F, t3579: F, t11737: F, t44857: F, t11791: F, t3490: F, t1227: F, t248: F, t3252: F, t3248: F, t11665: F, t11698: F, t11683: F, t11697: F, t11673: F, t11678: F, t11679: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t44936, t44938, t44951, t44953, t44965, t44968) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2187::<F>(t1174, t11760, t135, t11147, t3439, t11789, t820, t3577, t3579, t11737, t44857, t11791, t3490);
        let (t44972, t44976, t44982, t44985, t44988, t44991) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2188::<F>(t11789, t1227, t248, t3252, t3248, t11665, t11698, t11683, t11697, t3577, t11673, t11678, t11679);
    (t44936, t44938, t44951, t44953, t44965, t44968, t44972, t44976, t44982, t44985, t44988, t44991)
}
