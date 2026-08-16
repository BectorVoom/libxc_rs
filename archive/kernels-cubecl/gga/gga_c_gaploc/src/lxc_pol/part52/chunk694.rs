//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 694/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk694<F: Float>(t13402: F, t2488: F, t2487: F, t11172: F, t874: F, t1445: F, t597: F, t12991: F, t12997: F, t11595: F, t948: F, t2508: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13471 = t2488 * t13402;
    let t13472 = t2487 * t13471;
    let t13473 = F::cast_from(0.19171462976960374838e0_f64) * t13472;
    let t13474 = t11172 * t874;
    let t13475 = t1445 * t13474;
    let t13477 = F::cast_from(0.43710935587469654631e2_f64) * t597 * t13475;
    let t13478 = F::cast_from(0.59584149919750711116e-1_f64) * t12991;
    let t13480 = F::cast_from(0.11916829983950142223e0_f64) * t12997;
    let t13486 = t11595 * t948;
    let t13488 = F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t13486;
    (t13471, t13473, t13474, t13475, t13477, t13478, t13480, t13486, t13488)
}
