//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1100/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1100<F: Float>(t13949: F, t747: F, t1960: F, t2728: F, t3749: F, t2358: F, t39337: F, t42506: F, t42509: F, t44196: F, t44198: F, t44202: F, t44203: F, t47075: F, t47078: F, t841: F) -> (F, F) {
    let t47102 = t13949 * t747;
    let t47105 = t1960 * t3749 * t2728;
    let t47107 = t39337 * t2358;
    let t47108 = F::new(2.0) * t47107;
    let t47109 = -t47102 * t841 - t42506 - t42509 + F::new(2.0) * t44196 - t44198 + t44202 - t44203 + t47075 - t47078 + F::new(2.0) * t47105 - t47108;
    (t47108, t47109)
}
