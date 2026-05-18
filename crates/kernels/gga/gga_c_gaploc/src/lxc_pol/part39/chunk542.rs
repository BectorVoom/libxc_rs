//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 542/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk542<F: Float>(t9365: F, t2344: F, t2465: F, t2464: F, t2487: F, t1641: F, t3193: F, t447: F, t9176: F, t1445: F, t9215: F, t9211: F) -> (F, F, F, F, F, F, F) {
    let t9366 = F::new(0.38342925953920749676e0) * t9365;
    let t9367 = t2465 * t2344;
    let t9368 = t2464 * t9367;
    let t9369 = t2487 * t9368;
    let t9370 = F::new(0.85206502119823888169e-1) * t9369;
    let t9371 = t1641 * t3193;
    let t9373 = t9176 * t447;
    let t9374 = t1445 * t9373;
    let t9377 = t1445 * t9215;
    let t9380 = t1445 * t9211;
    (t9366, t9369, t9370, t9371, t9374, t9377, t9380)
}
