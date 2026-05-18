//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 905/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk905<F: Float>(t10151: F, t2464: F, t2465: F, t2487: F, t10417: F, t1415: F, t7030: F, t12960: F, t31051: F, t10473: F, t2478: F, t6576: F) -> (F, F, F, F) {
    let t41640 = t2487 * t2464 * t2465 * t10151;
    let t41643 = t1415 * t10417 * t7030;
    let t41645 = t31051 * t12960;
    let t41646 = F::new(0.19171462976960374838e1) * t41645;
    let t41649 = t6576 * t10473 * t2478;
    (t41640, t41643, t41646, t41649)
}
