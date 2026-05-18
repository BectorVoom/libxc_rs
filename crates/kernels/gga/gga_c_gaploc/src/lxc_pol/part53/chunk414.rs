//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 414/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk414<F: Float>(t1445: F, t3503: F, t2087: F, t3111: F, t3114: F, t3330: F, t3333: F, t3335: F) -> (F, F, F) {
    let t3504 = t1445 * t3503;
    let t3506 = F::new(0.69017266717057349418e1) * t2087 * t3504;
    let t3689 = t3335 + t3111 + t3330 - t3333 - t3114;
    (t3504, t3506, t3689)
}
