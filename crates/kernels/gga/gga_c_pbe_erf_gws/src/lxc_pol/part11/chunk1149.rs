//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1149/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1149<F: Float>(t1809: F, t47987: F, t639: F, t1815: F, t3473: F, t3553: F, t1620: F, t2677: F, t3465: F, t3562: F, t1022: F, t12493: F, t7853: F) -> (F, F, F, F) {
    let t48291 = F::new(32.0) / F::new(45.0) * t639 * t1809 * t47987;
    let t48295 = F::new(8.0) / F::new(15.0) * t639 * t1815 * t3473 * t3553;
    let t48299 = F::new(16.0) / F::new(9.0) * t1620 * t2677 * t3465 * t3562;
    let t48303 = F::new(256.0) / F::new(81.0) * t1620 * t7853 * t12493 * t1022;
    (t48291, t48295, t48299, t48303)
}
