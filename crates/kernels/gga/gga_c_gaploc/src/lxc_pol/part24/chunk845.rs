//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 845/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk845<F: Float>(t2464: F, t9367: F, t2487: F, t1641: F, t3193: F, t2462: F, t60: F) -> (F, F, F, F) {
    let t9368 = t2464 * t9367;
    let t9369 = t2487 * t9368;
    let t9370 = 0.85206502119823888169e-1 * t9369;
    let t9371 = t1641 * t3193;
    let t9419 = t60 * t2462;
    (t9368, t9370, t9371, t9419)
}
