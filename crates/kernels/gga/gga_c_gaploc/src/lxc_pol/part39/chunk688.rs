//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 688/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk688<F: Float>(t13041: F, t1445: F, t833: F, t2097: F, t3039: F, t3277: F, t12658: F, t3005: F, t3295: F, t9800: F, t11053: F, t9805: F, t1029: F, t9796: F, t12665: F, t12667: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13042 = t1445 * t13041;
    let t13044 = 0.43710935587469654631e2 * t833 * t13042;
    let t13045 = t3039 * t2097;
    let t13047 = 0.25025342966295298669e1 * t3277 * t13045;
    let t13050 = 0.11502877786176224903e1 * t12658;
    let t13052 = t3005 * t3295;
    let t13053 = t9800 * t13052;
    let t13054 = 0.19171462976960374838e1 * t13053;
    let t13055 = t11053 * t3295;
    let t13056 = t9805 * t13055;
    let t13057 = 0.11502877786176224903e1 * t13056;
    let t13058 = t1029 * t3295;
    let t13059 = t9796 * t13058;
    let t13060 = 0.76685851907841499353e0 * t13059;
    let t13061 = 0.59584149919750711116e-1 * t12665;
    let t13062 = 0.89376224879626066674e-1 * t12667;
    (t13042, t13044, t13045, t13047, t13050, t13052, t13054, t13055, t13057, t13058, t13060, t13061, t13062)
}
