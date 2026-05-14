//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 662/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk662<F: Float>(t1406: F, t9271: F, t10530: F, t584: F, t6574: F, t6575: F, t10215: F, t203: F, t3338: F, t447: F, t2366: F, t2754: F, t874: F, t6508: F, t2293: F, t986: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31054 = t1406 * t9271;
    let t31119 = t584 * t10530 * t6574;
    let t31356 = t1406 * t6575;
    let t31501 = t203 * t10215;
    let t31557 = t3338 * t447;
    let t31558 = t2366 * t31557;
    let t31585 = t2754 * t874;
    let t31586 = t6508 * t31585;
    let t31590 = t986 * t2293;
    (t31054, t31119, t31356, t31501, t31557, t31558, t31585, t31586, t31590)
}
