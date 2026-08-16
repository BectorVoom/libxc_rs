//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta700 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2284;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta700<F: Float>(t1174: F, t18206: F, t44562: F, t1227: F, t13969: F, t18958: F, t248: F, t45046: F, t5971: F, t15643: F, t5005: F, t1009: F, t18571: F, t1011: F, t1212: F, t3032: F, t65253: F, t3505: F, t3514: F, t15495: F, t4997: F, t15492: F, t5019: F, t15591: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t65914, t65920, t65935, t65952, t65955) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2284::<F>(t1174, t18206, t44562, t1227, t13969, t18958, t248, t45046, t5971, t15643, t5005, t1009, t18571);
        let (t65957, t65963, t65966, t65992, t65994, t65996) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2285::<F>(t1011, t1212, t65955, t3032, t65253, t3505, t3514, t15495, t4997, t15492, t5019, t15591);
    (t65914, t65920, t65935, t65952, t65955, t65957, t65963, t65966, t65992, t65994, t65996)
}
