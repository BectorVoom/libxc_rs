//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 581/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk581<F: Float>(t10156: F, t188: F, t10122: F, t475: F, t1445: F, t10152: F, t1: F, t3338: F, t106: F, t192: F) -> (F, F, F, F, F, F) {
    let t10485 = t188 * t10156;
    let t10488 = t10122 * t475;
    let t10489 = t1445 * t10488;
    let t10492 = t1445 * t10152;
    let t10495 = t3338 * t1;
    let t10496 = t10495 * t106;
    let t10497 = t10496 * t192;
    (t10485, t10489, t10492, t10495, t10496, t10497)
}
