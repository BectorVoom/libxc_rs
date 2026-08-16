//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1334/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1334<F: Float>(t2237: F, t29034: F, t1347: F, t24774: F, t3353: F, t8865: F, t3316: F, t8854: F, t20843: F, t4114: F, t4140: F, t6569: F) -> (F, F, F, F, F, F) {
    let t29036 = F::cast_from(0.16081979498692535067e2_f64) * t29034 * t2237;
    let t29038 = F::cast_from(2.0_f64) * t24774 * t1347;
    let t29040 = F::cast_from(4.0_f64) * t8865 * t3353;
    let t29042 = F::cast_from(2.0_f64) * t3316 * t8854;
    let t29044 = F::cast_from(2.0_f64) * t20843 * t4114;
    let t29046 = F::cast_from(1.0_f64) * t6569 * t4140;
    (t29036, t29038, t29040, t29042, t29044, t29046)
}
