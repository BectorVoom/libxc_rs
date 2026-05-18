//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 700/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk700<F: Float>(t13041: F, t1445: F, t833: F, t2097: F, t3039: F, t3277: F, t12658: F, t3005: F, t3295: F, t9800: F, t11053: F, t9805: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13042 = t1445 * t13041;
    let t13044 = F::new(0.43710935587469654631e2) * t833 * t13042;
    let t13045 = t3039 * t2097;
    let t13047 = F::new(0.25025342966295298669e1) * t3277 * t13045;
    let t13050 = F::new(0.11502877786176224903e1) * t12658;
    let t13052 = t3005 * t3295;
    let t13053 = t9800 * t13052;
    let t13054 = F::new(0.19171462976960374838e1) * t13053;
    let t13055 = t11053 * t3295;
    let t13056 = t9805 * t13055;
    (t13042, t13044, t13045, t13047, t13050, t13052, t13054, t13055, t13056)
}
